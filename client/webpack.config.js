const path = require('path');

const { LoaderOptionsPlugin } = require('webpack');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { BundleAnalyzerPlugin } = require('webpack-bundle-analyzer');
const TerserPlugin = require('terser-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

let distPath = path.join(__dirname, 'dist');

module.exports = (env, argv) => {
    return {
        watch: true,
        entry: {
            bootstrap: "./www/bootstrap.js",
            worker: "./www/worker.js",
            main: "./www/styles/main.scss",
        },
        output: {
            filename: '[name].js',
            path: __dirname + "/dist"
        },

        // Enable sourcemaps for debugging webpack's output.
        devtool: "source-map",

        devServer: {
            contentBase: distPath,
            compress: true,
            port: 9000
        },

        mode: argv.mode || 'production',
        watch: argv.mode !== 'production',

        optimization: {
            minimizer: [new TerserPlugin({
                sourceMap: true,
                cache: true,
                parallel: true,
            })],
        },

        performance: {
            hints: false
        },

        resolve: {
            extensions: [".ts", ".tsx", ".js", ".json"],
            alias: {
                //"react": "preact-compat",
                //"react-dom": "preact-compat",
                modernizr$: path.resolve(__dirname, 'src/.modernizrrc'),
            }
        },

        plugins: [
            new BundleAnalyzerPlugin(),
            new MiniCssExtractPlugin({
                // Options similar to the same options in webpackOptions.output
                // both options are optional
                filename: "[name].css",
                chunkFilename: "[id].css"
            }),
            new WasmPackPlugin({
                crateDirectory: "./bin/app",
                //extraArgs: "--target web",
                watchDirectories: [
                    path.resolve(__dirname, "src")
                ]
            }),
            new WasmPackPlugin({
                crateDirectory: "./bin/native_worker",
                extraArgs: "--target no-modules",
                watchDirectories: [
                    path.resolve(__dirname, "src")
                ]
            }),
            new CopyWebpackPlugin({
                patterns: [
                    { from: './www/index.html', to: distPath + '/index.html' }
                ]
            }),
        ],

        module: {
            rules: [
                // All files with a '.ts' or '.tsx' extension will be handled by 'awesome-typescript-loader'.
                { test: /\.tsx?$/, loader: "awesome-typescript-loader" },

                // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
                { enforce: "pre", test: /\.js$/, loader: "source-map-loader" },
                {
                    test: /\.(sa|sc|c)ss$/,
                    use: [
                        {
                            loader: MiniCssExtractPlugin.loader,
                            options: {
                                // you can specify a publicPath here
                                // by default it use publicPath in webpackOptions.output
                                //publicPath: '../'
                            }
                        },
                        'css-loader',
                        'postcss-loader',
                        'sass-loader',
                    ],
                },
                {
                    test: /\.modernizrrc$/,
                    loader: ["modernizr-loader", "json-loader"]
                },
                {
                    test: /\.(woff(2)?|ttf|eot|svg)(\?v=\d+\.\d+\.\d+)?$/,
                    use: [
                        {
                            loader: 'file-loader',
                            options: {
                                name: '[name].[ext]',
                                outputPath: 'fonts/'
                            }
                        }
                    ]
                }
            ]
        },
    };
};