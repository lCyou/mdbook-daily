(function () {
  document
    .querySelector('.sidebar-scrollbox')
    .scroll(
      0,
      document.querySelector('.sidebar .active')
      .getBoundingClientRect()
      .top - 50
    );
})();
