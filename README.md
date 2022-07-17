# Twitimer
A tool for scheduled tweet.

## Overview
Twitimer is a tool for scheduled tweet with 
send and delete time set by user in advance.  

However, twitimer is not a daemon program.
Thus, twitimer should be triggered by outside job scheduler
like cron and systemd timer.

## Installation
We currently only provide binary executable file in GitHub Release page for **Linux amd64** architecture.  
As a result, the installation instructions below are only tested for **Linux amd64** architecture.  

Please build it yourself and carefully adapt the instructions below if your system architecture is not **Linux amd64**.
### Download and add to PATH
#### Linux amd64
```shell
$ sudo wget 'https://github.com/iochen/twitimer/releases/download/v0.0.1(release)/twitimer' -O /usr/bin/twitimer
$ sudo chmod +x /usr/bin/twitimer
```
#### Other architectures
```shell
$ cargo install twitimer
```

### Get your Twitter credentials
If you are not a Twitter developer, go and apply for one.  
After the successful application:
1. Go to [create a new app](https://developer.twitter.com/en/portal/apps/new).
2. Save **API Key** and **API Key Secret**, and ignore **Bearer Token**
3. Go to app settings, click **Set up** to set **User authentication settings**
4. Toggle **OAuth 1.0a** to **ON** and check **App permissions** as **Read and write**
5. Fill in the left required fields, since they are unrelated to twitimer, any value can be okay.
6. Click **Save** to save the configuration
7. Go to **Keys and tokens** under your app and click **Generate** to generate **Access Token and Secret**.
8. Save **Access Token** and **Access Token Secret**
### Init twitimer
Execute and answer all questions to generate twitimer database.
```shell
$ twitimer init
```
Notice: **Consumer {Key, Secret}** are **API Key** and **API Key Secret** above  

Notice: **Access {Key, Secret}** are **Access Token** and **Access Token Secret** above

### Add to cron
```shell
$ crontab -e
```
Then insert a new line like below
```cron
* * * * * /usr/bin/twitimer cron
```

### Post your scheduled tweet
```shell
$ twitimer new
```
1. Enter your message with newline and ^D (or Ctrl+D) to finish
2. Enter the start time of your tweet with one of [three valid time formats listed at the end of the file](#time-format)

### Wait and check
Just wait and check whether everything works well as expected!


## Environment variable
You can set environment variable(s) shown below to configure twitimer
- `TWITIMER_DB` to specify twitimer database path

## Time format
#### Timestamp version
```
{UNIX timestamp}
```
Where
- `{UNIX timestamp}` stands for UNIX timestamp, example: `1431648000`, `1658035509`

#### Absolute version
```
{Year}-{Month}-{Day} {Hour}:{Minute}:{Second} {Timezone}
```
Where
- `{Year}` stands for year, example: `2022`, `2032`, `02033`
- `{Month}` stands for month, example: `02`, `3`, `11`
- `{Day}` stands for day, example: `04`, `9`, `27`
- `{Hour}` stands for hour, example: `06`, `7`, `11`
- `{Minute}` stands for minute, example: `08`, `4`, `59`
- `{Second}` stands for second, example: `05`, `0`, `32` 
- `{Timezone}` stands for timezone, example: `+9`, `-7`, `0`

example: `2022-02-12 13:23:45 +9`

#### Relative version
```
now+{Hours}h{Minutes}m
```
or
``` 
now+{Hours}h
```
or
``` 
now+{Minutes}m
```

Where
- `{Hours}` stands for hours left till now, example: `0`, `3`, `72`
- `{Minutes}` stands for minutes left till now, example: `0`, `5`, `86`

example: `now+3h` or `now+2h30m` or `now+87m`


## Tips
- Although we will try our best, we still cannot 100% guarantee the compatibility between versions even between `x.y.z` and `x.y.(z+1)`

## LICENSE
MIT License