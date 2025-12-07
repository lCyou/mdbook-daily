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

(function () {
  var as = document.querySelectorAll('a.header');
  newList.setAttribute('class', 'innerLink');
  for (var i = 1, l = as.length; i < l; i += 1) {
    var a = as[i];
    var label = a.innerText;
    var href = a.getAttribute('href');
    var newAnchor = document.createElement('a');
    newAnchor.setAttribute('href', href);
    newAnchor.innerHTML = label;
    var newItem = document.createElement('li');
    newItem.appendChild(newAnchor);
    newList.appendChild(newItem);
  }
  document.querySelector('.sidebar .active').appendChild(newList);
})();