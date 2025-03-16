const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.[contenthash].js',
    clean: true,
  },
  mode: process.env.NODE_ENV === 'production' ? 'production' : 'development',
  devtool: process.env.NODE_ENV === 'production' ? 'source-map' : 'eval-source-map',
  experiments: {
    asyncWebAssembly: true,
  },
  resolve: {
    extensions: ['.js', '.wasm'],
    alias: {
      // Add an alias to make imports cleaner
      'wasm-game-of-life': path.resolve(__dirname, '../pkg')
    }
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html',
      inject: 'body',
    }),
    new CopyPlugin({
      patterns: [
        { from: 'assets', to: 'assets', noErrorOnMissing: true },
      ],
    }),
  ],
  devServer: {
    static: {
      directory: path.join(__dirname, 'dist'),
    },
    compress: true,
    port: 8080,
    hot: true,
    open: true,
    client: {
      overlay: true,
    },
  },
};