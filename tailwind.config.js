
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

