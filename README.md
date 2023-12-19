# 🕷️ Aragog 🕷️

##### A basic implementation of a web crawler. No Acromantulas hurt in making of this project.

Usage (if built): 
```console
./aragog --url <URL> --depth <N>
```  

Or (directly from the repo): 
```console
cargo run --release -- --url <URL> --depth <N>
```

##### About

Again, this is a very barebones implementation. Aragog assumes you only care about http(s) as the protocol. It also doesn't really do anything with the webpages, just downloads the http files. However in my opinion with the way the code is written, it should be relatively easy to plug in a parser function that extracts the relevant data you're looking for. 

The original design for this assumed I would be doing the url parsing, however upon research, I figured out that using the [`webpage`](https://myoctocat.com/assets/images/base-octocat.svg) crate made much more sense for me, as it takes care of the boilerplate part. Initially, I tried several other crates, including `reqwest` and `hyper`, but `webpage` seemed most ergonomic for me. Could also theoretically use `serde`  and `surf`, but `webpage` worked well enough for me so I didn't delve further. 

Initially the first version of the refined algorithm looked like this:

* `function` -> visit a webpage
  * save the webpage
  * for each link in the webpage, call `function` recursively

While this does the job on first look, there are a couple problems you need to look for in order to get the best experience: 

1. You need to figure out how to save a webpage, what do you need for it? You can put everything in a database, but I just recursively create folders for each website. 
2. More importantly: You need to check for if the URL goes to another domain or if it's within the same webpage. Otherwise, the logic of the recursiveness doesn't work properly. 

##### Potential improvements
1. Actually write some kind of parser (e.g. extract the titles of the most important paragraphs?)
2. Don't just discriminate based on base domain, but figure out if a link inside a page points to the same page (e.g. example.com/article#someheader should be considered the same as example.com/article and shouldn't be parsed)
3. Move from the recursive architecture to an imperative one, to avoid stack issues (this should only be a problem if you're parsing enormous amount of webpages)
4. Implement multithreading - In theory this is very logical as each page's parser can be made to run on a different thread, and theoretically works well with recursive nature of the program too. In practice, data coordination between threads is a clear problem, if you want to avoid repeated waste work.
