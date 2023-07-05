<p align="center">
  <img src="https://github.com/m1guelpf/tinyvector/assets/23558090/512ff4ad-49fd-43ec-b3bd-57365b920078" alt="tinyvector logo">
</p>

<p align="center">
    <b>tinyvector - a tiny embedding database in pure Rust</b> <br /><br />
    <a href="https://crates.io/crates/tinyvector"><img src="https://img.shields.io/crates/v/tinyvector" ></a> <a href="https://github.com/m1guelpf/tinyvector/actions/workflows/build"><img src="https://github.com/m1guelpf/tinyvector/actions/workflows/build.yaml/badge.svg" ></a>  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" ></a>
</p>
<hr />

## ✨ Features
- **Tiny**: It's in the name. It's literally just an axum server. Extremely easy to customize, around 600 lines of code.
- **Fast**: Tinyvector _should_ have comparable speed to advanced vector databases when it comes on small to medium datasets, and slightly better accuracy.
- **Vertically Scales**: Tinyvector stores all indexes in memory for fast querying. Very easy to scale up to 100 million+ vector dimensions without issue.
- **Open Source**: MIT Licensed, free forever.

### Soon
- **Powerful Queries**: Allow filtering by the provided vector metadata without slowing the search down.
- **Integrated Models**: Soon you won't have to bring your own vectors, just generate them on the server automaticaly. Aiming to support support SBert, Hugging Face models, OpenAI, Cohere, etc.
- **Typescript/Python Libraries**: Should be able to auto-generate pretty good clients using the included OpenAPI schema.

## 🚀 Getting Started

### 🐳 Docker

We provide a lightweight Docker container that you can run anywhere. It only takes one command to get up and running with the latest changes:

```sh
docker run \
  -p 8000:8000 \
  ghcr.io/m1guelpf/tinyvector:edge
```

> **Note**
> When running via Docker Compose or Kubernetes, make sure to bind a volume to `/tinyvector/storage` for persistence. This is handled automatically in the command above.

### 🛠️ Building from scratch

You can build tinyvector from the latest tagged release by running `cargo install tinyvector` (you might need to [install Rust](https://rustup.rs/) first). Then, run `tinyvector` to start up the server.
 
You can also build it from the latest commit by cloning the repo and running `cargo build --release`, and run it with `./target/release/tinyvector`.

## 💡 Why use tinyvector?

Most vector databases are overkill for simple setups. For example:
- Using embeddings to chat with your documents. Most document search is nowhere close to what you'd need to justify accelerating search speed with [HNSW](https://github.com/nmslib/hnswlib) or [FAISS](https://github.com/facebookresearch/faiss).
- Doing search for your website or store. Unless you're selling 1,000,000 items, you don't need Pinecone.

## 🧩 Embeddings?

Embeddings are a way to compare similar things, in the same way humans compare similar things, by converting text into a small list of numbers. Similar pieces of text will have similar numbers, different ones have very different numbers.

Read OpenAI's [explanation](https://platform.openai.com/docs/guides/embeddings/what-are-embeddings).

## 🙏 Acknowledgements

- Will Depue's [tinyvector](https://twitter.com/willdepue/status/1675796236304252928) (python+sqlite+numpy) inspired me to build a vector database from scratch (and borrow the name). Will also contributed plenty of ideas to optimize performance.

## 📄 License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
