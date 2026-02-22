# MyDramaList API (Unofficial)

A high-performance, asynchronous REST API wrapper for [MyDramaList](https://mydramalist.com) built with **Rust**, **Axum**, and **Utoipa**.

Gotta be real, I don't know why I actually made this...

## API Documentation

Once the server is running, you can explore the full API specification and test endpoints through the Scalar UI:

👉 **[http://localhost:3000/scalar](http://localhost:3000/scalar)**

### Key Endpoints

| Method | Path                     | Description                                                                      |
| :----- | :----------------------- | :------------------------------------------------------------------------------- |
| `GET`  | `/titles/search`         | Advanced search for Dramas and Movies with filters for genre, year, and country. |
| `GET`  | `/people/search`         | Search for actors and crew using nationality and gender filters.                 |
| `GET`  | `/articles/search`       | Search MDL editorials, news, and recaps.                                         |
| `GET`  | `/api-docs/openapi.json` | Raw OpenAPI 3.1 JSON specification.                                              |

## ⚠️ Disclaimer

This is an **unofficial** wrapper. It relies on web scraping techniques to retrieve data.

- It is not affiliated with MyDramaList.
- Changes to the MyDramaList website structure may break the parser.
- Use responsibly and respect MDL's terms of service and robots.txt.

## License

This project is licensed under the MIT License.
