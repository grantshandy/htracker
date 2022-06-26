const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  transpileDependencies: true,
  productionSourceMap: false,
  pages: {
    index: {
      entry: 'src/pages/index/main.js',
      title: 'htracker'
    },
    login: {
      entry: 'src/pages/login/login.js',
      title: 'Login to htracker'
    },
    register: {
      entry: 'src/pages/register/register.js',
      title: 'Register for htracker'
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
