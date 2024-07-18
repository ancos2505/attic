document.addEventListener("DOMContentLoaded", function () {
  const prefersDarkScheme = window.matchMedia(
    "(prefers-color-scheme: dark)"
  ).matches;
  if (prefersDarkScheme) {
    document.body.classList.add("dark-theme");
    // localStorage.setItem("theme", "dark");
  } else {
    document.body.classList.add("light-theme");
    // localStorage.setItem("theme", "light");
  }

  const themeToggleButton = document.getElementById("theme-toggle");

  // Check if there's a saved theme preference
  // if (localStorage.getItem("theme") === "dark") {
  //   document.body.classList.add("dark-theme");
  // } else {
  //   document.body.classList.add("light-theme");
  // }

  themeToggleButton.addEventListener("click", function () {
    if (document.body.classList.contains("light-theme")) {
      document.body.classList.remove("light-theme");
      document.body.classList.add("dark-theme");
      // localStorage.setItem("theme", "dark");
    } else {
      document.body.classList.remove("dark-theme");
      document.body.classList.add("light-theme");
      // localStorage.setItem("theme", "light");
    }
  });
});

// document.addEventListener('DOMContentLoaded', function() {
//     const prefersDarkScheme = window.matchMedia('(prefers-color-scheme: dark)').matches;
//     if (prefersDarkScheme) {
//        document.body.classList.add('dark-mode');
//     }
//    });
