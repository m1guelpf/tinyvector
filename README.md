<p align="center">
    <b>tinyvector - the tiny, least-dumb, speedy vector embedding database</b>. <br />
    🦀 rewrite of <a href="https://github.com/0hq/tinyvector">0hq's tinyvector</a>
</p>

## Features
- __Tiny__: It's in the name. It's literally just an axum server. Extremely easy to customize, around 600 lines of code.
- __Fast__: Tinyvector will have comparable speed to advanced vector databases when it comes to speed on small to medium datasets.
- __Vertically Scales__: Tinyvector stores all indexes in memory for fast querying. Very easy to scale up to 100 million+ vector dimensions without issue.
- __Open Source__: MIT Licensed, free forever.

### Soon
- __Integrated Models__: Soon you won't have to bring your own vectors, just generate them on the server automaticaly. Will support SBert, Hugging Face models, OpenAI, Cohere, etc.
- __Python/JS Client__: We'll add a comprehensive Python and Javascript package for easy integration with tinyvector in the next two weeks.

## We're better than ...

In most cases, most vector databases are overkill for something simple like:
1. Using embeddings to chat with your documents. Most document search is nowhere close to what you'd need to justify accelerating search speed with [HNSW](https://github.com/nmslib/hnswlib) or [FAISS](https://github.com/facebookresearch/faiss).
2. Doing search for your website or store. Unless you're selling 1,000,000 items, you don't need Pinecone.
3. Performing complex search queries on a very large database. Even if you have 2 million embeddings, this might still be the better option due to vector databases struggling with complex filtering. Tinyvector doesn't support metadata/filtering just yet, but it's very easy for you to add that yourself.

## Embeddings?

What are embeddings?

> As simple as possible: Embeddings are a way to compare similar things, in the same way humans compare similar things, by converting text into a small list of numbers. Similar pieces of text will have similar numbers, different ones have very different numbers.

Read OpenAI's [explanation](https://platform.openai.com/docs/guides/embeddings/what-are-embeddings).

## License

[MIT](./LICENSE)
