const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const distPath = path.resolve(__dirname, "dist");

module.exports = (env, argv) => {
    return {
        devServer: {
            static: {
                directory: distPath,
            },
            compress: true,
            port: 8000,
            historyApiFallback: true,
        },
        entry: './bootstrap.js',
        output: {
            path: distPath,
            filename: "richard-chukwu.js",
            publicPath: '/richard-chukwu/',
            webassemblyModuleFilename: "richard-chukwu.wasm",
            clean: true,
        },
        experiments: {
            asyncWebAssembly: true, // Enable async WebAssembly
            // syncWebAssembly: true, // Use this if you need synchronous WebAssembly (not recommended)
        },
        module: {
            rules: [
                {
                    test: /\.s[ac]ss$/i,
                    use: [
                        MiniCssExtractPlugin.loader,
                        'css-loader',
                        'postcss-loader',
                        'sass-loader',
                    ],
                },
                {
                    test: /\.wasm$/,
                    type: 'webassembly/async', // Handle .wasm files as async WebAssembly
                },
            ],
        },
        plugins: [
            new MiniCssExtractPlugin({
                filename: '[name].css',
            }),
            new CopyWebpackPlugin({
                patterns: [
                    { from: './static', to: distPath },
                ],
            }),
            new WasmPackPlugin({
                crateDirectory: ".",
                extraArgs: "--no-typescript",
            }),
        ],
    };
};
