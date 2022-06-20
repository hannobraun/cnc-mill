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

- **OnShape:** https://www.onshape.com/<br />
  Looks highly professional. Would certainly be an interesting learning experience.
- **SketchUp:** https://sketchup.com/<br />
  Also looks interesting, although the website presents architecture and furniture use cases, versus the mechanical assemblies showcased on the OnShape website. Hard to say how relevant this will be for this project.
- **TinkerCAD:** https://www.tinkercad.com/<br />
  Looks least interesting, judging from the website, as it stresses the beginner/education use case. Doesn't mean that it won't be more than capable enough for this project though.

### Configuration

I believe that the following machine configurations are the strongest contenders:

- **Moving gantry:** Tool moves in all 3 axes.
- **Fixed gantry:** Tool moves in x and z axes, table moves in y axis.

I have ruled out more exotic configurations for my first build (despite having lots of ideas), to reduce overall risk.

References:

- https://cncchronicle.com/fixed_or_moving_gantry_for_cnc_router/<br />
  Compares the two configuration. Has a nice comparison table further down.

### Control Software

On the controller side I have the following priorities:

- Resist the temptation to do any custom software, or otherwise use something exotic. This might be an option in later builds, but I want to keep it simple for the first version.
- Be controllable from a regular PC, without requiring one to run. I don't want to add a screen/keyboard/mouse to the bill of materials, but I also don't want to dedicate a whole computer to control it.

I found the following options:

- **LinuxCNC:** https://linuxcnc.org/<br />
  Runs on the Raspberry Pi:
  http://linuxcnc.org/docs/stable/html/getting-started/getting-linuxcnc.html

  The download page talks about interface cards. Not sure what specifically is required, but I found this list:
  http://wiki.linuxcnc.org/cgi-bin/wiki.pl?LinuxCNC_Supported_Hardware

  I haven't done much research, but the interface cards I saw were quite expensive. Overall, I get the impression that LinuxCNC is not suited for a budget-sensitive build.

- **Machinekit:** https://www.machinekit.io/<br />
  I found it hard to understand whether this is suitable. What I can gather is that it runs on the BeagleBone Black: https://www.machinekit.io/docs/getting-started/machinekit-images-for-bbb/

  No idea what else is required to make it work. The information presented is not very approachable, and I'm not sure how much of it is outdated.

- **grbl:** https://github.com/gnea/grbl<br />
  I've often heard about this being used for the kind of small-scale CNC machine I'm aiming to build. It's confusing though. There are two different repositories, neither actively developed, and lots of forks.

- **grblHAL:** https://www.grbl.org/<br />
  Fork of gbrl, for 32-bit MCUs.

- **FluidNC:** https://github.com/bdring/FluidNC<br />
  Looks promising. Has a list of supported hardware: https://github.com/bdring/FluidNC/wiki/Hardware-that-Runs-FluidNC

Based on my cursory research into this area, I think this might be the wrong approach. It might be better to search for easily available CNC controllers, and figure out which software to use for them from there.

Host-side control software:
- https://github.com/Denvi/Candle
- https://github.com/winder/Universal-G-Code-Sender
- https://github.com/terjeio/ioSender

### Controller Boards

I'm currently operating under the assumption that an open-loop control system using stepper motors will be used, for cost reasons.

There is a huge number of options available. A lot of them are pricy enough to take up the majority of the available budget. For that reason, I've focused on lower-priced options.

TinyG control boards, which include steppers drivers, but lack something to control the spindle:

- https://synthetos.myshopify.com/products/tinyg<br />
  Not the cheapest option ($129.99), but open source and well-documented. Includes 4 stepper drivers (2.5 amps).
- https://synthetos.myshopify.com/collections/assembled-electronics/products/gshield-v5<br />
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
    - Makerbase MKS A4988: https://www.aliexpress.com/item/32888457440.html<br />
      2.0A max
    - Makerbase MKS TMC2208: https://www.aliexpress.com/item/32888980385.html<br />
      2.0A max
    - Makerbase MKS TMC2209: https://www.aliexpress.com/item/33043140087.html<br />
      2.5A max
    - Makerbase MKS TMC2225: https://www.aliexpress.com/item/4001149124672.html<br />
      2.0A max
    - Makerbase MKS TMC2226: https://www.aliexpress.com/item/1005002669282600.html<br />
      2.5A max
  - Makerbase MKS TMC2160: https://www.aliexpress.com/item/1005004044381878.html<br />
    4.33A max
  - Makerbase MKS TMC2160-OC: https://www.aliexpress.com/item/4000185818422.html<br />
    4.33A max, extra cooling

### Online Shops

I'm looking into two groups of online shops now:

- Local (i.e. European) shops, because that gives me a minimum of hassle (shipping, customs).
- Shops on platforms like AliExpress or eBay with low prices, as that might be necessary to meet my budget.

European shops:

- cnc-technics: https://shop.cnc-technics.de/<br />
  Have lots of relevant products, but probably too high-end for my budget.
- DOLD Mechatronik: https://dold-mechatronik.de/<br />
  Have a lot of different stuff, including aluminium in various sizes. At least some categories seems to be high-priced, relative to the available budget.

AliExpress:

- Bulk Man 3D: https://bulkman3d.com/, https://www.aliexpress.com/store/1752067<br />
  Large selection of lots of things I'm going to need.
- Makerbase: https://www.aliexpress.com/store/1047297<br />
  Control boards mostly seem not applicable, but the stepper drivers are very interesting.

eBay:

- https://www.ebay.de/str/motormall<br />
  Chinese shop, ships from Europe. Has spindles and VFDs.

### Other Machines

It's going to make sense to take inspiration from other machines at some point. Here are some links:

- OpenBuilds: https://openbuilds.com/<br />
  Lots of machines of all kinds.
- MPCNC: https://docs.v1engineering.com/mpcnc/intro/<br />
  Interesting design. Less traditional than what I have in mind.
- Tormach xsTECH: https://tormach.com/machines/routers/xstech-router.html<br />
  Really interesting machine, in terms of configuration, form factor, and enclosure. Pretty much what I want to build, but hopefully I can fit some more rigid components into the budget.
- PrintNC: https://wiki.printnc.info/en/home<br />
  Moving gantry design with a frame based on steel tubing.
- Millennium Mill: https://www.reddit.com/r/MilleniumMachines/<br />
  C-frame mill based on aluminium extrusion.
- ULTIMATE Bee: https://bulkman3d.com/knowledge-base/ultimate-bee/<br />
  Classic moving gantry design, with what looks like high-quality components.

### Spindle

After an initial round of research, I'm now focusing on water-cooled spindles with a minimum of around 1kW power and 24k max RPM, for the following reasons:

- According to information I found online, 1kW and 24k max RPM should be good for milling aluminium.
- Noise is an important concern, and water-cooled spindles are quieter than air-cooled ones.
- It seems like the only real downsides are price and complexity. Prices are surprisingly affordable, from what I can see, and I've been told the complexity isn't too bad.

#### Manufacturers

In articles I've read, Huanyang has been called out as a quality brand, and I can find affordable offers with EU-based inventory. Unless the budget turns out to be really tight, or some other information comes to light, I might just go with that.

#### Collets

I've been told (from a trusted source) to get at least ER16 or greater, as ER11 is too limiting.

#### References

- https://mellowpine.com/cnc/how-to-choose-a-cnc-spindle/<br />
  Has very specific recommendations.
- https://mellowpine.com/cnc/best-cnc-spindles/<br />
  Presents some specific spindles and what they're suitable for.
- https://en.wikipedia.org/wiki/Collet#ER_collets<br />
  Just some background info on ER collets, for the mechanically challenged (like me).

### Spindle: Counterpoint

This is some information I've come across, that runs counter to my previous research on spindles above. The way I arrived there went roughly like that:

1. The cheap CNC mills sold by Sainsmart have up to 400W spindles, there seems to be wide agreement that they're not great for aluminium, and on top of that, they are marketed as "CNC engravers".
2. Credible-looking information online tells me I need at least 0.8 - 1 kW for aluminium. If the Sainsmart mills are not good for aluminium, needing twice as much to be good at aluminium makes sense.

Now I'm finding counter-examples though: machines that are used to mill aluminium, but are much less powerful. Some examples:

- [**PocketNC:**](https://pocketnc.com/) Both the V2-10 and the V2-50 come with a 200W spindle[^1][^2]. And yet it seems capable milling aluminium and more. This video is very interesting: https://youtu.be/7YfRNZbfjaY?t=326
- [**Nomad:**](https://carbide3d.com/nomad/) Only has a 70W spindle. It's easy to find videos of it milling aluminium, but in the one's I've seen, either the sound is covered by a voiceover, or it sounds horribly chattery. So not a strong case, but interesting none the less.

I think the PocketNC is a strong example here. It's obviously not capable of great speeds, and it starts chattering if the settings are too aggressive. But still, it seems to be capable of producing aluminium parts.

Given the size constraints of the machine, being able to use a smaller spindle would be very attractive.

[^1]: https://cdn.shopify.com/s/files/1/0077/5477/6623/files/V2-10_Spec_V05.pdf?v=1611173337
[^2]: https://cdn.shopify.com/s/files/1/0077/5477/6623/files/V250CHKCHBSpecSheet.pdf?v=1624559427
[^3]: https://youtu.be/vMY06dzf7UA?t=127

### Power Supply

As far as I can see, the machine needs the following kinds of power:

- **230V AC:** Since I'm in Europe, this is the input I'm dealing with. It's needed by the VFD for the spindle, the DC power supply, possibly the water pump.
- **3.3V - 5V DC:** This is the typical range for microcontrollers and many other kinds of electronics.
- **higher-voltage DC:** The stepper motors are going to need DC at a higher voltage than the controller.

So I'm going to need a power supply that turns AC into DC, and possibly something else to step the DC up or down to meet the different requirements. As for the power supply, I've often seen [Meanwell](https://www.meanwell.com/) in various sets and such. Unless there's a good reason not to, I might just stick to that.


## Design

Here are some high-level design decisions I've made. None of them are final, and they might still change as I do more research:

- **Configuration:** Fixed gantry. The advantages are just too big, and I think I can live with a smaller work area in one axis.
- **Size:** 40x40x40 cm³ or thereabouts. Those are outer dimensions. If it turns out that this leaves not enough working area, I can go a bit larger, especially in height.
- **Spindle:** Water-cooled AC spindle, 1 kW or more, 24.000 max. RPM
- **Axis motors:** Stepper motors. I haven't really looked into servos, but seeing how many machines run just fine with steppers, I'm pretty confident they will work for me. An open-loop control system will also reduce cost and complexity.
