module.exports = {
  purge: {
    enabled: true,
    content: ["./index.html", "./src/**/*.js", "./src/**/*.vue"],
  },
  future: {
    removeDeprecatedGapUtilities: true,
    purgeLayersByDefault: true,
  },
};
