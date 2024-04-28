
// tailwind.config.js
module.exports = {
    content: [
        "./src/**/*.rs", // Path to your Rust component files
        "./static/**/*.html" // Any other HTML files
      ],
    darkMode: 'class', // or 'media'
    theme: {
      extend: {
        colors: {
            'rich-blue': '#243c5a',
            'rich-accent': '#FF6B6B',
            'primary': '#f5f5f5',
            'background': '#121212',
            // Define other colors as needed
          },
      },
    },
    // Ensure other needed configurations are set
  };

  // module.exports = {
  //   content: [
  //     "./src/**/*.rs", // Path to your Rust component files
  //     "./static/**/*.html" // Any other HTML files
  //   ],
  //   darkMode: ["class", '[data-mode="theme-dark"]'],
  //   theme: {
  //     extend: {
  //       transitionDuration: {
  //         0: "0ms",
  //         400: "400ms",
  //         450: "450ms",
  //       },
  //       width: {
  //         128: "32rem",
  //       },
  //     },
  //     themes: {
  //       dark: {
  //         white: "#fff",
  //         black: "#000",
  //         "text-main": "#fff",
  //         "accent-primary": "#2563EB",
  //         "accent-tinted": "#3897e9",
  //         "accent-shaded": "#1f7dcf",
  //         "bg-main": "#111",
  //         "bg-secondary": "#191919",
  //         "muted-main": "#E6E6E6",
  //         "muted-secondary": "#CCCCCC",
  //       },
  //     },
  //     colors: {
  //       white: "#000",
  //       black: "#fff",
  //       "accent-primary": "#228be6",
  //       "accent-tinted": "#3897e9",
  //       "accent-shaded": "#1f7dcf",
  //       "text-main": "#111",
  //       "bg-main": "#fff",
  //       "bg-secondary": "#333",
  //       "muted-main": "#fff",
  //       "muted-secondary": "#fff",
  //     },
  //   },
  //   transitionProperty: {
  //     scale: "scale",
  //   },

  // };