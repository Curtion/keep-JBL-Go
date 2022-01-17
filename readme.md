jbl-go 蓝牙音箱在没有声音输出时会自动休眠，此程序尝试自动输出人耳听不到的音频来保持音箱长久工作。

音频文件通过以下命令生成：

`ffmpeg -f lavfi -i "sine=frequency=40000:sample_rate=48000:duration=1" -af "volume=-60dB" -c:a pcm_s16le slient.wav`

参考文章：
https://www.simaek.com/archives/15/
