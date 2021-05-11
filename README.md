# Tensorflow Classifier Rust API

Small project to learn a bit of API building with Rust. A simple classifier with one endpoint returning the prediction from a .pb model (here mobilenet).

```bash
$ cargo run 
$ curl -X GET -H "Content-type: application/json" -d '{"path": ":assets/dog.jpg"}' http://localhost:9090/classes
```