import './static/style.scss';

import("./pkg").then(module => {
  module.run_app();
});

document.addEventListener('copy', function(e){
  e.stopImmediatePropagation();
  return true;
}, true);

document.addEventListener('paste', function(e){
  e.stopImmediatePropagation();
  return true;
}, true);
