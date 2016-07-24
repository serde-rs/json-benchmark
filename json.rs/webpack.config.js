const path               = require('path');
const webpack            = require('webpack');

const PROD = true;


module.exports = {
    entry   : {
        bundle  : './src/benches.js'
    },


    output  : {
        path        : path.join( __dirname, 'target' ),
        filename    : 'bundle.js',
        publicPath  : '/'
    },

    plugins : PROD ? [
        new webpack.optimize.UglifyJsPlugin( {
            minimize    : true,
            compress    : {
                warnings    : false,
                drop_console: true
            }
        } ),
    ] : [],

    node    : {
        fs          : 'empty',
        console     : false,
        global      : true,
        process     : true,
        Buffer      : false,
        setImmediate: false
    },
};
