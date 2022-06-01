# CNC Mill

## Goal

I want to have a 3-axis CNC mill. I've considered buying one, but everything I can find violates one or more of the following constraints:

- **Budget:** It's going to be my first CNC mill, so spending a lot of money doesn't make a whole lot of sense. The main purpose is to learn, and figure out what I actually need in a CNC mill. For that reason, I'd like to keep the budget below 1000€.
- **Capability:** At the same time, I don't want to buy a machine that I will outgrow within the first week. It should be capable of cutting aluminium.
- **Space:** I'm severely limited on space. It needs to comfortably fit on a desk.
- **Environment:** I don't have a proper workshop, unfortunately. The machine will need to run in my apartment, preferably without making my neighbors hate me.

There simply doesn't seem to be a machine like this, which is not a surprise. For this reason, I'd like to try and build my own, purpose-made for my use case.

I suspect it might simply be impossible to fulfill all of these requirements at within limited budget. If that is the case, I'm looking forward to learning what the limitations are specifically. I'm also aware that there's a high risk that a self-built machine will fall short in some or all of these areas. I think it's still likely to be a better investment, for the learning experience alone.


## Status

As of this writing, I've literally just started. This README exists, and nothing else.


## License

This project is open source. All documents, design files, and software in this repository are available under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with them, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md


## Research Notes

These are the notes from my research process.

### CAD Software

I'd love to use [Fornjot](https://www.fornjot.app/) for this, but it'll be a while before it's ready for a project like this. I hope that I can migrate the project to Fornjot eventually.

Normally, I'd favor open source software, but since migration to Fornjot at a later stage is planned anyway, I don't see that as a priority right now. I've used OpenSCAD, FreeCAD, SolveSpace, and other open source options in the past, and I'd like to take this opportunity to try something new.

I'm focusing the research on options that are free (or have a free tier) and support Linux:

- **OnShape:** https://www.onshape.com/
  Looks highly professional. Would certainly be an interesting learning experience.
- **SketchUp:** https://sketchup.com/
  Also looks interesting, although the website presents architecture and furniture use cases, versus the mechanical assemblies showcased on the OnShape website. Hard to say how relevant this will be for this project.
- **TinkerCAD:** https://www.tinkercad.com/
  Looks least interesting, judging from the website, as it stresses the beginner/education use case. Doesn't mean that it won't be more than capable enough for this project though.

### Configuration

I believe that the following machine configurations are the strongest contenders:

- **Moving gantry:** Tool moves in all 3 axes.
- **Fixed gantry:** Tool moves in x and z axes, table moves in y axis.

I have ruled out more exotic configurations for my first build (despite having lots of ideas), to reduce overall risk.

References:

- https://cncchronicle.com/fixed_or_moving_gantry_for_cnc_router/
  Compares the two configuration. Has a nice comparison table further down.

### Control Software

On the controller side I have the following priorities:

- Resist the temptation to do any custom software, or otherwise use something exotic. This might be an option in later builds, but I want to keep it simple for the first version.
- Be controllable from a regular PC, without requiring one to run. I don't want to add a screen/keyboard/mouse to the bill of materials, but I also don't want to dedicate a whole computer to control it.

I found the following options:

- **LinuxCNC:** https://linuxcnc.org/
  Runs on the Raspberry Pi:
  http://linuxcnc.org/docs/stable/html/getting-started/getting-linuxcnc.html

  The download page talks about interface cards. Not sure what specifically is required, but I found this list:
  http://wiki.linuxcnc.org/cgi-bin/wiki.pl?LinuxCNC_Supported_Hardware

  I haven't done much research, but the interface cards I saw were quite expensive. Overall, I get the impression that LinuxCNC is not suited for a budget-sensitive build.

- **Machinekit:** https://www.machinekit.io/
  I found it hard to understand whether this is suitable. What I can gather is that it runs on the BeagleBone Black: https://www.machinekit.io/docs/getting-started/machinekit-images-for-bbb/

  No idea what else is required to make it work. The information presented is not very approachable, and I'm not sure how much of it is outdated.

- **grbl:** https://github.com/gnea/grbl
  I've often heard about this being used for the kind of small-scale CNC machine I'm aiming to build. It's confusing though. There are two different repositories, neither actively developed, and lots of forks.

- **grblHAL:** https://www.grbl.org/
  Fork of gbrl, for 32-bit MCUs.

- **FluidNC:** https://github.com/bdring/FluidNC
  Looks promising. Unclear though, which hardware it runs on. (Well, ESP32, obviously, but I mean which specific boards I could use.)

Based on my cursory research into this area, I think this might be the wrong approach. It might be better to search for easily available CNC controllers, and figure out which software to use for them from there.

Host-side control software:
- https://github.com/Denvi/Candle
- https://github.com/winder/Universal-G-Code-Sender
- https://github.com/terjeio/ioSender

### Controller Boards

I'm currently operating under the assumption that an open-loop control system using stepper motors will be used, for cost reasons.

There is a huge number of options available. A lot of them are pricy enough to take up the majority of the available budget. For that reason, I've focused on lower-priced options.

TinyG control boards, which include steppers drivers, but lack something to control the spindle:

- https://synthetos.myshopify.com/products/tinyg
  Not the cheapest option ($129.99), but open source and well-documented. Includes 4 stepper drivers (2.5 amps).
- https://synthetos.myshopify.com/collections/assembled-electronics/products/gshield-v5
  Cheaper alternative to the TinyG ($49.99), which requires an Arduino to work. I happen to have an Arduino Due lying around, so that might work out well. Includes 3 stepper drivers (2.5 amps).

Both of those boards don't seem to be available in Europe, so the real cost might be significantly higher, with shipping and customs duties.

SainSmart sells the controller boards for their low-cost CNC machines separately:
- https://www.sainsmart.com/collections/genmitsu-cnc-replacement-upgrade-parts/products/genmtisu-grbl-controller-board-for-3018-prover-3018-mx3
- https://www.sainsmart.com/collections/genmitsu-cnc-replacement-upgrade-parts/products/controller-board-for-genmtisu-cnc-router-3018-3018-pro-1810-rpo

They are much cheaper (40-50€) and can control a spindle. One of them is limited to 1.5-2amps for the stepper motors, the other doesn't specify. Since the SainSmart machines are pretty weak, it's doubtful that the controller board supports any motors (axis or spindle) that would be a significant upgrade over them.

Here's another board, linked to grblHAL:
https://www.tindie.com/products/philba/grblhal-bob-unkit-for-teensy-41-t41u5xbb/

Costs around 50€, plus shipping from the US. Requires a Teensy 4.1, which is readily available locally, for under 30€. Doesn't include stepper drivers, but can control a spindler with an external VFD.

References:

- https://www.cnccookbook.com/cnc-controller-software-drivers-boards/
  Overview over some available options.

### Stepper Drivers

Makerbase: https://www.aliexpress.com/store/1047297
  - standard(?) breakout board pinout
    - Makerbase MKS A4988: https://www.aliexpress.com/item/32888457440.html
      2.0A max
    - Makerbase MKS TMC2208: https://www.aliexpress.com/item/32888980385.html
      2.0A max
    - Makerbase MKS TMC2209: https://www.aliexpress.com/item/33043140087.html
      2.5A max
    - Makerbase MKS TMC2225: https://www.aliexpress.com/item/4001149124672.html
      2.0A max
    - Makerbase MKS TMC2226: https://www.aliexpress.com/item/1005002669282600.html
      2.5A max
  - Makerbase MKS TMC2160: https://www.aliexpress.com/item/1005004044381878.html
    4.33A max
  - Makerbase MKS TMC2160-OC: https://www.aliexpress.com/item/4000185818422.html
    4.33A max, extra cooling

### Further Areas of Research

These are just some notes, not an exhaustive list.

- Stepper drivers, if using a board that doesn't have them built-in.
- VFDs, if using a board that requires them to control the spindle motor.
- Power supplies.
- Cooling. Active or passive cooling might be required for the control electronics.
