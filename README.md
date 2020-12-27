`tmlatestbackup`
=================

*Really simple CLI app to obtain the date of the last Mac OS Time Machine backup.*

![Status](http://www.borja.glezseoane.es/img/project-status-label-development.svg "Status: development")



## Use

The normal use of the tool is very simple. You only need to run:

```sh
tmlatestbackup
```

And obtain the date of the last Time Machine backup with format like: `2018-11-21-102312`.



## Motivation

On my Mac OS machine I have some scripts that cannot work as agents because it is prevented by System Integrity Protection, which only allows granting permissions to binary programs. That, coupled with the fact that I wanted to learn the Rust language, motivated me to write this simple program in order to have the conflicting step in a compiled program to authorize it and can maintain my script routine working elegantly —and not with pseudo-compiled shell scripts inserted in a compiled language program—. However, at the end of the day `tmlatestbackup` is a potentially useful generic purpose program.
