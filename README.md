# photosort

![Static Badge](https://img.shields.io/badge/Codeberg-Robangsan-555599?style=for-the-badge&logo=codeberg&logoColor=%23eee&labelColor=%23337&link=https%3A%2F%2Fcodeberg.org%2FRobangsan%2Fphotosort)
![GitHub License](https://img.shields.io/github/license/Robangsan/photosort?style=for-the-badge)

**NOTE: _photosort_ development has been migrated to [Codeberg](https://codeberg.org/Robangsan/photosort). Any future changes will not be reflected in this repository!**

_photosort_ is a quick tool I wrote for sorting photo and video files (although _technically_ it should be able to sort other kinds of files) into folders based on their creation date according to their EXIF data or, in its absence, the file metadata.

The `photosort` command will, by default, sort all photo and video files in the chosen path with these extensions:
* JPG/JPEG
* PNG
* TIF/TIFF
* MP4
* AVI

The input path can be specified by passing it as an argument with the `-i` option:
```
photosort -i path
```

To sort the files in the current path, you can use `photosort -i .`

These can be specified with the `-e` option, adding one for each extension. For instance:
```
photosort -e jpg -e bmp -e webm
```

The output folder is the same as the input by default, unless another one is specified with the `-o` option.
```
photosort input_folder -o output_folder
```

The files can be copied instead of moved if the `-c` option is passed.

By default, the tool will sort the files into folders with the year of their creation (e.g. if the photo was created in 2020, it will be moved into the `\2020\`  folder). An additional layer of sorting can be added where the files will be distributed into subfolders using the `-s` option (e.g. if the photo was created in March 2020, it will be moved into the `\2020\3\` folder).

The tool will look for files only in the specified folder, unless the `-r` option is specified. Then it will look recursively into subfolders.

You can see all the available options with `photosort -h` or `photosort --help`.

### **CAUTION: This application is in alpha state and is largely UNTESTED. I'm not responsible for any data loss that may occur because of its usage. Use at your own risk!**


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
