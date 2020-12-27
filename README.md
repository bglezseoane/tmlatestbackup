`tmlatestbackup`
=================

*Really simple CLI app to obtain the date of the last Mac OS Time Machine backup.*

![Status](http://www.borja.glezseoane.es/img/project-status-label-development.svg "Status: development")


## Install

The recommended installation option is use Homebrew with the command:

```sh
brew install bglezseoane/tap/tmlatestbackup
```

You can only install the program with Cargo, with the command:

```sh
cargo install tmlatestbackup
```



## Use

The normal use of the tool is very simple. You only need to run:

```sh
tmlatestbackup
```

And obtain the date of the last Time Machine backup with format like: `2018-11-21-102312`.


### Achieve permissions against Mac OS System Integrity Protection

If your current **terminal app** has **full disk access** and you use `tmlatestbackup` **since it**, the tool will works fine. If you only want to work with `tmlatestbackup` **since your terminal** or since scripts used by you since your terminal, **the above is sufficient** and the following steps are irrelevant to your use case.

Otherwise, if you want to use the tool since an script routine as **`launchd` agent**, the tool is going to fail due to System Integrity Protection.

Mac OS System Integrity Protection block `tmlatestbackup` because the tool try to access protected Time Machine features and it doesn't inherit permissions —**using it since the terminal, inherit terminal granted ones**—. To achieve permissions, you need to grant them adding the binary (`/usr/local/bin/tmlatestbackup`) to the list of applications with full disk access —System Preferences > Security & Privacy > Full Disk Access > Add—.

The path `/usr/local/bin/tmlatestbackup` could be different if you have installed with Cargo. Any case, you can check it with `which tmlatestbackup`.

If your agent only needs to run the tool and no more stuff, you can add the following lines to the agent `Info.plist` and it will work (after previously indicated step):

```xml
<key>ProgramArguments</key>
    <array>
      	<string>/usr/local/bin/tmlatestbackup</string>
    </array>
```

For any reason, to work with the tool since the `launchd` agent and **integrated in a shell script**, you need to always run it as `open -a '/usr/local/bin/tmlatestbackup'` and not directly as a command (e.g. `/usr/local/bin/tmlatestbackup`). I.e., if you use, e.g., the following agent specification...

```xml
<key>ProgramArguments</key>
    <array>
      	<string>/Users/You/scripts/script_which_calls_tmlatestbackup.sh</string>
    </array>
```

... you need to run `tmlatestbackup` as follows:

```sh
# 'script_which_calls_tmlatestbackup.sh'
# ...

open -a '/usr/local/bin/tmlatestbackup'
# And not '/usr/local/bin/tmlatestbackup

# ...
```

When run with `open -a`, the command don't return an error code if the launched application fails, so in order to integrate this step in a well designed script, the next approach is recommended:

```sh
# ...

# Create temporary file to save the output of the 'open' command
TMP_OPEN_TMLATESTBACKUP_STDOUT=$(mktemp -t open_tmlatestbackup)

# Run and wait
open -W -a '/usr/local/bin/tmlatestbackup' \
	--stdout "$TMP_OPEN_TMLATESTBACKUP_STDOUT" \
	--stderr "$TMP_OPEN_TMLATESTBACKUP_STDOUT"

# Check and act attending to the process success
if grep -Fxq 'Error' "$TMP_OPEN_TMLATESTBACKUP_STDOUT" \
	|| ! grep -Fxq 'Process finished successfully.' "$TMP_OPEN_TMLATESTBACKUP_STDOUT"
then
	>&2 echo "Error using 'tmlatestbackup'."
	rm "$TMP_OPEN_TMLATESTBACKUP_STDOUT"
	exit 1 
fi

# Clean
rm "$TMP_OPEN_TMLATESTBACKUP_STDOUT"

# ...
```



## Motivation

On my Mac OS machine I have some scripts that cannot work as agents because it is prevented by System Integrity Protection, which only allows granting permissions to binary programs. That, coupled with the fact that I wanted to learn the Rust language, motivated me to write this simple program in order to have the conflicting step in a compiled program to authorize it and can maintain my script routine working elegantly —and not with pseudo-compiled shell scripts inserted in a compiled language program—. However, at the end of the day `tmlatestbackup` is a potentially useful generic purpose program.
