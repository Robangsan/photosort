# photosort

_photosort_ is a quick tool I wrote for sorting photo and video files (although _technically_ it should be able to sort other kinds of files) into folders based on their creation date according to their EXIF data or, in its absence, the file metadata.

The `photosort` command will, by default, sort all photo and video files in the current folder with these extensions:
* JPG/JPEG
* PNG
* TIF/TIFF
* MP4
* AVI

These can be specified with the `-e` option, adding one for each extension. For instance:
```
photosort -e jpg -e bmp -e webm
```

The path where the tool will look for files can be passed as an argument, like this:
```
photosort dir\another_dir
```

By default, the tool will sort the files into folders with the year of their creation (e.g. if the photo was created in 2020, it will be moved into the `\2020\`  folder). An additional layer of sorting can be added where the files will be distributed into subfolders using the `-s` option (e.g. if the photo was created in March 2020, it will be moved into the `\2020\3\` folder).

The tool will look for files only in the specified folder, unless the `-r` option is specified. Then it will look recursively into subfolders.

You can see all the available options with `photosort -h` or `photosort --help`.

**CAUTION: This application is in alpha state and is largely UNTESTED. I'm not responsible for any data loss that may occur because of its usage. Use at your own risk!**