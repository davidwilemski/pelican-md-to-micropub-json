# pelican-md-to-micropub

This tool takes a markdown file that is formatted to [work with Pelican](https://docs.getpelican.com/en/stable/content.html) (e.g. has post properties at the top) and converts it into a JSON document that is compatible with the Micropub format.

I haven't yet supported all of the properties allowed Pelican or in Micropub – just those that were required to support reading the markdown files that my personal website. It currently ignores the `Author` and `Category` properties by assuming there is a single author (meant for blogs) and that the post type will be `h-entry`.

## Usage:

```
$ target/debug/pelican-md-to-micropub - | jq
Title: test title
Date: 2020-04-04 15:30
Tags: tag1, tag2
Slug: test-slug

This is my test content
```

Results in output of:

```
{
  "type": "h-entry",
  "properties": {
    "name": [
      "test title"
    ],
    "mp-slug": [
      "test-slug"
    ],
    "content": [
      {
        "html": "This is my test content\n"
      }
    ],
    "published": [
      "2020-04-04 15:30"
    ],
    "category": [
      "tag1",
      "tag2"
    ]
  }
}
```

You can also provide a file as the input.
