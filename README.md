<p align="center">
  <img src="https://github.com/m1guelpf/tinyvector/assets/23558090/512ff4ad-49fd-43ec-b3bd-57365b920078" alt="tinyvector logo">
</p>


<p align="center">
    <b>tinyvector - the tiny, least-dumb, speedy vector embedding database</b>. <br /><br />
    <a href="https://crates.io/crates/tinyvector"><img src="https://img.shields.io/crates/v/tinyvector" ></a> <a href="https://github.com/m1guelpf/tinyvector/actions/workflows/build"><img src="https://github.com/m1guelpf/tinyvector/actions/workflows/build.yaml/badge.svg" ></a>  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" ></a>
</p>
<hr />

## Features
- __Tiny__: It's in the name. It's literally just an axum server. Extremely easy to customize, around 600 lines of code.
- __Fast__: Tinyvector should have comparable speed to advanced vector databases when it comes on small to medium datasets, and slightly better accuracy.
- __Vertically Scales__: Tinyvector stores all indexes in memory for fast querying. Very easy to scale up to 100 million+ vector dimensions without issue.
- __Open Source__: MIT Licensed, free forever.

### Soon
- __Integrated Models__: Soon you won't have to bring your own vectors, just generate them on the server automaticaly. Will support SBert, Hugging Face models, OpenAI, Cohere, etc.
- __Python/JS Client__: We'll add a comprehensive Python and Javascript package for easy integration with tinyvector in the next two weeks.

## Why use tinyvector?

Most vector databases are overkill for simple setups. For example:
- Using embeddings to chat with your documents. Most document search is nowhere close to what you'd need to justify accelerating search speed with [HNSW](https://github.com/nmslib/hnswlib) or [FAISS](https://github.com/facebookresearch/faiss).
- Doing search for your website or store. Unless you're selling 1,000,000 items, you don't need Pinecone.

## Embeddings?

Embeddings are a way to compare similar things, in the same way humans compare similar things, by converting text into a small list of numbers. Similar pieces of text will have similar numbers, different ones have very different numbers.

Read OpenAI's [explanation](https://platform.openai.com/docs/guides/embeddings/what-are-embeddings).

## Acknowledgements

- Will Depue's [tinyvector](https://twitter.com/willdepue/status/1675796236304252928) (python+sqlite+numpy) inspired me to build a vector database from scratch (and borrow the name). Will also contributed plenty of ideas to optimize performance.

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
