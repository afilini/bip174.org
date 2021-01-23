import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap';

import 'bootstrap-icons/font/bootstrap-icons.css';

import './static/style.scss';

import("./pkg").then(module => {
  module.run_app();
});
