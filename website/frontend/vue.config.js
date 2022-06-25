const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  transpileDependencies: true,
  pages: {
    index: {
      entry: 'src/pages/index/main.js',
      title: 'Htracker Home'
    },
    login: {
      entry: 'src/pages/login/login.js',
      title: 'Login to Htracker'
    }
  },
  chainWebpack : (config) => {
    config.output.filename('[name].js');
  },
  css: {
    extract: {
      filename: '[name].css',
      chunkFilename: '[name].css'
    }
  }
})
