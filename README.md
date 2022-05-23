# Motivation

Sometimes when watching videos or streams on my computer there is a sudden noise which is very loud and painful.

In addition, sometimes I am on a voice call with friends and _we_ are loud.

I wanted a way to be able to lower these loud sounds a bit. That is, I wished for a way to either lower my own microphone's peaks, or to lower all peaks that I hear through my earbuds.

I searched online and didn't find much. Voicemeeter Banana was probably the best one, but it's a very large and complex program with an overwhelming user interface. I also had trouble getting it working and was experiencing issues with static. I just wanted something simpler.

# Description

This program can hopefully be that. The intention is for the interface to be extremely boring. Simply select your input device, output device, and threshold, and hit start. That's it. Then all sound piped through the input device will be limited (according to the threshold) and sent to the output device.

# Compiling From Source

Install Rust if you don't have it already and then clone the repository and run `cargo run` in the root folder.

# Setup

If you don't have a virtual audio cable to pipe audio through already, then you'll probably need to download one.

[This one](https://vb-audio.com/Cable/) works fine for me and is the one I'm currently using. So I'd recommend downloading that.

Then, you want to make sure all your sample rates match (otherwise they'll become out of sync and start dropping audio). For example, if you plan to use the program with your microphone and computer sound, then go to your Sound Settings on your computer, select all the relevant sound devices, and go to their properties, and make sure all the sample rates match up.

I personally made all of my devices 16 bit, 48000 Hz like this:

![Audio Device Setup](https://i.imgur.com/AfUgm2X.png)

# Using The Program

Let's say you wanted to limit all incoming sound on your computer. Here's how you would do that:

First, change your playback device on your computer from your speakers / headphones to `Cable Input`:

![Playback Devices](https://i.imgur.com/2Wqvc9H.png)

Then, run `audio-limiter.exe`. If you're on Windows you can download a [binary here](https://github.com/dylagit/audio-limiter/releases). If you're on another platform, follow the `Compiling From Source` instructions above. When you open the program, it should look something like this:

![Audio Limiter](https://i.imgur.com/hPTh2bw.png)

Next, you'll want to change the `Input Device` to `Cable Output`.

Then, you'll want to change the `Output Device` to your speakers / headphones. Then click Start. If it works, the `Start` button's text should change to `Stop`. If it didn't, then something went wrong. It should now look like this:

![Audio Limiter Settings](https://i.imgur.com/ECPy4fH.png)

Now start listening to music / videos / whatever, and adjust the `Threshold` until it all sounds normal. Now, if a loud sound occurs, its magnitude should be blunted.

# Disclaimer

I have no idea what I'm doing. The main reason why I made this open-source with a detailed tutorial is that I'm hoping someone who _does_ know what they're doing comes along and either suggestions improvements, or ideally just submits a pull request and improves things.

As it stands, I'm still brand new to Rust, have no education whatsoever with regard to digital audio processing, and only started learning about making user interfaces in Rust yesterday.

That is, this program barely works, if at all. I would love for it to be much better, so I am extremely receptive to any proposed changes.

I would love if there was a way for the program to automatically determine the threshold instead of having to adjust it all the time, but I'm not sure how to do that, or if it's even possible. In addition, it might be nice to have some additional parameters for how blunted the sound should become, and things of that nature. But, as stated, I'm a complete noob, so I'll need a lot of help if you would like those features as well.