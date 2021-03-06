# 下载

build 文件夹有 Windows x86_x64 和 Arm MacOS 俩可执行文件可以直接下载，其余平台请自行构建。

# 说明

jbl-go 蓝牙音箱在没有声音输出时会自动休眠，此程序尝试自动输出人耳听不到(其实可以听到)的音频来保持音箱长久工作。

已经测试低频声音并不能阻止休眠，所以程序会播放 1 秒的有声音频，如果感觉声音较大可以考虑自行生成音频文件再构建。

音频文件通过以下命令生成：

`ffmpeg -f lavfi -i "sine=frequency=30000:sample_rate=48000:duration=1" -af "volume=-30dB" -c:a pcm_s16le slient.wav`

目前这个音量在我这里运行得很好，既几乎听不到声音，音箱又不会休眠。

参考文章：
https://www.simaek.com/archives/15/

# 提示

jbl-go 用的 LM48511 芯片,给想要硬改的朋友一个信息。

# 构建

使用`cargo build --release`构建程序

# 运行

需要自动解决不同操作系统下自启的问题，例如 Windows 可以使用`任务计划程序`，Mac 可以使用`自动操作`, Linux 可以使用`systemd`
