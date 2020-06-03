const path = require('path');

const { LoaderOptionsPlugin } = require('webpack');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { BundleAnalyzerPlugin } = require('webpack-bundle-analyzer');
const TerserPlugin = require('terser-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = (env, argv) => {
    return {
        watch: true,
        entry: {
            bootstrap: "./www/bootstrap.js",
        },
        output: {
            filename: '[name].js',
            path: __dirname + "/dist"
        },

        // Enable sourcemaps for debugging webpack's output.
        devtool: "source-map",

        devServer: {
            contentBase: path.join(__dirname, 'dist'),
            compress: true,
            port: 9000
        },

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
            new HtmlWebpackPlugin({
                title: 'Custom template',
                // Load a custom template (lodash by default)
                template: 'www/index.html'
            }),
            new MiniCssExtractPlugin({
                // Options similar to the same options in webpackOptions.output
                // both options are optional
                filename: "[name].css",
                chunkFilename: "[id].css"
            }),
            new LoaderOptionsPlugin({
                options: {
                    posthtml(ctx) {
                        return {
                            parser: require('posthtml-pug'),
                            plugins: [
                                require('posthtml-bem')()
                            ]
                        }
                    }
                }
            }),
            new WasmPackPlugin({
                crateDirectory: "."
            })
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
                    test: /\.html$/,
                    use: [
                        {
                            loader: 'html-loader',
                            options: { minimize: true }
                        },
                        {
                            loader: 'posthtml-loader'
                        }
                    ]
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