# rust-light-server
Light web server using rust, Just like node [http-server](https://www.npmjs.com/package/http-server) from example of [rust book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)

## Usage
rust-light-server [OPTIONS] [PATH]

[PATH] - Server path which contains html files to serve

## Options
| Option | Description | Default value
| --- | --- | ---
| --port | Port number for server to listen | 7878
| --gzip / -g | enable gzip compression | |
| --help / -h | Displays list of options available  

## Example 
> rust-light-server --port 8089 --gzip c:\demo\webpages\


it will start the server on port 8089 and serve the files from webpages folder
web url will be http://127.0.0.1:8089