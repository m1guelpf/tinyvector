<p align="center">
    <b>tinyvector - the tiny, least-dumb, speedy vector embedding database</b>. <br />
    Now in Rust! 🦀
</p>
<p align="center">
    A Rust rewrite of <a href="https://github.com/0hq/tinyvector">Will Depue's Tinyvector</a> <br />
</p>

## Features

- **Tiny**: It's in the name. It's just a Flask server, SQLite DB, and Numpy indexes. Extremely easy to customize, under 500 lines of code.
- **Fast**: Tinyvector already beats other advanced vector databases when it comes to speed on small to medium datasets.
- **Vertically Scales**: Tinyvector stores all indexes in memory for fast querying. Very easy to scale up to 100 million+ vector dimensions without issue.
- **Open Source**: MIT Licensed, free forever.

### Soon

- **Powerful Queries**: Tinyvector is being upgraded with full SQL querying functionality, something missing from most other databases.
- **Integrated Models**: Soon you won't have to bring your own vectors, just generate them on the server automaticaly. Will support SBert, Hugging Face models, OpenAI, Cohere, etc.
- **Python/JS Client**: We'll add a comprehensive Python and Javascript package for easy integration with tinyvector in the next two weeks.

## Embeddings?

What are embeddings?

> As simple as possible: Embeddings are a way to compare similar things, in the same way humans compare similar things, by converting text into a small list of numbers. Similar pieces of text will have similar numbers, different ones have very different numbers.

Read OpenAI's [explanation](https://platform.openai.com/docs/guides/embeddings/what-are-embeddings).

## License

[MIT](./LICENSE)
