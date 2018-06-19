import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap';

import 'bootstrap-icons/font/bootstrap-icons.css';

import './style.scss';

import("./pkg").then(module => {
  module.run_app();
});
