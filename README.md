# CNC Mill

## Goal

I want to have a 3-axis CNC mill. I've considered buying one, but everything I can find violates one or more of the following constraints:

- **Budget:** It's going to be my first CNC mill, so spending a lot of money doesn't make a whole lot of sense. The main purpose is to learn, and figure out what I actually need in a CNC mill. For that reason, I'd like to keep the budget below 1000€.
- **Capability:** At the same time, I don't want to buy a machine that I will outgrow within the first week. It should be capable of cutting aluminium.
- **Space:** I'm severely limited on space. It needs to comfortably fit on a desk.
- **Environment:** I don't have a proper workshop, unfortunately. The machine will need to run in my apartment, preferably without making my neighbors hate me.

There simply doesn't seem to be a machine like this, which is not a surprise. For this reason, I'd like to try and build my own, purpose-made for my use case.

I suspect it might simply be impossible to fulfill all of these requirements within the limited budget. If that is the case, I'm looking forward to learning what the limitations are specifically. I'm also aware that there's a high risk that a self-built machine will fall short in some or all of these areas. I think it's still likely to be a better investment, for the learning experience alone.


## Status

I've been doing research for a while now, and have made some preliminary design decisions. All of that is documented below.


## License

This project is open source. All documents, design files, and software in this repository are available under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with them, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md


## Design Decisions

Here are some high-level design decisions I've made. None of them are final, and they might still change as I do more research:

- **Configuration:** Fixed gantry. The advantages are just too big, and I think I can live with a smaller work area in one axis.
- **Size:** 40x40x40 cm³ or thereabouts. Those are outer dimensions. If it turns out that this leaves not enough working area, I can go a bit larger, especially in height.
- **Spindle:** Water-cooled AC spindle, 1 kW or more, 24.000 max. RPM. There is some doubt about whether such a powerful spindle is really necessary. For now, I'm just going ahead with it, and will revisit if that escalates the budget or size requirements too much.
- **Axis motors:** Stepper motors. I haven't really looked into servos, but seeing how many machines run just fine with steppers, I'm pretty confident they will work for me. An open-loop control system will also reduce cost and complexity.


## Research Notes

These are the notes from my research process.

### Other Machines

It's going to make sense to take inspiration from other machines at some point. Here are some links:

- [**OpenBuilds**](https://openbuilds.com/): Lots of machines of all kinds.
- [**MPCNC**](https://docs.v1engineering.com/mpcnc/intro/): Interesting design. Less traditional than what I have in mind.
- [**Tormach xsTECH**](https://tormach.com/machines/routers/xstech-router.html): Really interesting machine, in terms of configuration, form factor, and enclosure. Pretty much what I want to build, but hopefully I can fit some more rigid components into the budget.
- [**PrintNC**](https://wiki.printnc.info/en/home): Moving gantry design with a frame based on steel tubing.
- [**Millennium Mill**](https://www.reddit.com/r/MilleniumMachines/): C-frame mill based on aluminium extrusion.
- [**ULTIMATE Bee**](https://bulkman3d.com/knowledge-base/ultimate-bee/): Classic moving gantry design, with what looks like high-quality components.
- [**PocketNC**](https://pocketnc.com/): 5-axis desktop CNC mill. Interesting, in that it is used for milling aluminium (easy to find examples on YouTube), but has a relatively weak spindle (200W).
- [**Nomad**](https://carbide3d.com/nomad/): Pretty close to what I would like to build, in regards to the configuration, form factor, and enclosure. Notable for its weak spindle (70W).

### Configuration

I believe that the following machine configurations are the strongest contenders:

- **Moving gantry**: Tool moves in all 3 axes.
- **Fixed gantry**: Tool moves in x and z axes, table moves in y axis.

I have ruled out more exotic configurations for my first build (despite having lots of ideas), to reduce overall risk.

References:

- https://cncchronicle.com/fixed_or_moving_gantry_for_cnc_router/<br />
  Compares the two configurations. Has a nice comparison table further down.

### Online Shops

I'm looking into two groups of online shops:

- Local (i.e. European) shops, because that gives me a minimum of hassle (shipping, customs).
- Shops on platforms like AliExpress or eBay with low prices, as that might be necessary to meet my budget.

European shops:

- [**cnc-technics:**](https://shop.cnc-technics.de/): Have lots of relevant products, but probably too high-end for my budget.
- [**DOLD Mechatronik**](https://dold-mechatronik.de/): Have a lot of different stuff, including aluminium in various sizes. At least some categories seems to be high-priced, relative to the available budget.
- [**Sorotec**](https://www.sorotec.de/shop/): Large selection of all kinds of stuff required for CNCs.
- [**MISUMI**](https://de.misumi-ec.com/): Large selection of materials, mechanical components, and much more. Lots of options for modifying material and aluminium extrusion parts.

AliExpress:

- [**Bulk Man 3D**](https://www.aliexpress.com/store/1752067) ([also have a website](https://bulkman3d.com/)): Large selection of lots of things I'm going to need.
- [**Makerbase**](https://www.aliexpress.com/store/1047297): Control boards mostly seem not applicable, but the stepper drivers are very interesting.

eBay:

- [**motor-mall**](https://www.ebay.de/str/motormall): Chinese shop, ships from Europe. Has spindles and VFDs.
- Those seems to be essentially the same:
  - [**kuku081**](https://www.ebay.de/str/kuku081)
  - [**kuku281**](https://www.ebay.de/str/kuku218)
  - [**kuku86**](https://www.ebay.de/str/kuku86)
  - [**powave21**](https://www.ebay.de/str/powace21)
  - [**rattmmotor**](https://www.ebay.de/str/rattmmotor)
  - [**rattmmotor88**](https://www.ebay.de/str/rattmmotor88)
  - [**RATTM MOTOR CNC**](https://www.ebay.de/str/rattmmotorcnc)
- [**chinacnczone-de**](https://www.ebay.de/str/cnczonedd)

### Spindle

I see two potentially viable ways to go, regarding the spindle:

- Cheaper DC spindle, around 500W.
- Mid-range AC spindle, possibly water-cooled.

#### DC Spindles

Advantages of DC spindles:

- There are examples of machines with relatively weak spindles that can definitely mill aluminium, although not fast and with not-so-great surface finish. It might be enough for my needs.
- Cheaper than AC spindles.
- More compact than AC spindles. That doesn't just go for the spindle itself, but also for the hardware needed to control it (a possibly bulky VFD, in the case of AC spindles). Given the size constraints, this is very attractive.
- No dealing with AC power.

Examples of machines with relatively weak DC spindles:

- **PocketNC**: Both the V2-10 and the V2-50 come with a 200W spindle[^1][^2]. And yet it seems capable milling aluminium and more. This video is very interesting: https://youtu.be/7YfRNZbfjaY?t=326
- **Nomad**: Only has a 70W spindle. It's easy to find videos of it milling aluminium, but in the one's I've seen, either the sound is covered by a voiceover, or it sounds horribly chattery. So not a strong case, but interesting none the less.

I think the PocketNC is a strong example here. It's obviously not capable of great speeds, and it starts chattering if the settings are too aggressive. But still, it seems to be capable of producing aluminium parts.

Examples of DC spindles:

- [104W, 10.8k RPM, ER8, ~140€](https://www.ebay.de/itm/384842723224); includes driver and bracket
- [400W, 12k RPM, ER8, ~130€](https://www.ebay.de/itm/255388710895); includes driver and bracket
- [400W, 12k RPM, ER8, ~140€](https://www.ebay.de/itm/384842725572); includes driver and bracket
- [500W, 12k RPM, ER11, ~120€](https://www.ebay.de/itm/403510912800); includes driver and bracket
- [500W, 12k RPM, ER11, ~60€](https://www.ebay.de/itm/174570637963); includes driver and bracket
- [600W, 12k RPM, ER11, ~85€](https://www.ebay.de/itm/384858278032); motor only

[^1]: https://cdn.shopify.com/s/files/1/0077/5477/6623/files/V2-10_Spec_V05.pdf?v=1611173337
[^2]: https://cdn.shopify.com/s/files/1/0077/5477/6623/files/V250CHKCHBSpecSheet.pdf?v=1624559427

#### AC Spindles

Advantages of AC spindles:

- They are simply more powerful. I found information, that 1kW power and 24k max RPM should be good for milling aluminium, and I think everything in that range is an AC spindle.
- Water-cooled AC spindles are pretty common. Those are quieter and more long-lived, and I've been told that water cooling isn't too bad, from a complexity perspective.
- The Chinese ones are still surprisingly affordable, although I have no idea how the quality compares to more expensive ones.

Notes:

- In articles I've read, Huanyang has been called out as a quality brand, and I can find affordable offers with EU-based inventory. Unless the budget turns out to be really tight, or some other information comes to light, I might just go with that.
- Some spindles come in a rectangular form factor, with mounting holes included. Seems more convenient than the round ones.

Examples:

- Air-cooled:
  - [0.8kW, 24k RPM, ER11, ~240€](https://www.ebay.de/itm/174956638303); VFD included
  - [0.8kW, 24k RPM, ER11, ~240€](https://www.ebay.de/itm/185102648864); VFD specified at 1.5kW
  - [2.2kW, 24k RPM, ER20, ~220€](https://www.ebay.de/itm/171841127523); VFD included
  - [2.2kW, 24k RPM, ER20, ~220€](https://www.ebay.de/itm/183100123168); VFD included; not sure, if there's any difference to the previous one
- Water-cooled:
  - [1.5kW, 24k RPM, ER16, ~155€](https://www.ebay.de/itm/185467595787); spindle only
  - [2.2kW, 24k RPM, ER20, ~320€](https://www.ebay.de/itm/185340019340); full set: spindle, VFD, water pump, holder, collets
  - [2.2kW, 24k RPM, ER20, ~350€](https://www.ebay.de/itm/185467757422); VFD included
  - [3.0kW, 24k RPM, ER20, ~165€](https://www.ebay.de/itm/185102654786); spindle only
  - [3.0kW, 24k RPM, ER20, ~285€](https://www.ebay.de/itm/174974637566); VFD included

#### Collets

I've been told (from a trusted source) to get at least ER16 or greater, as ER11 is too limiting. It seems like ER11 and ER20 are the most common sizes, so ER20 should be the one to look for.

#### References

- https://mellowpine.com/cnc/how-to-choose-a-cnc-spindle/<br />
  Has very specific recommendations.
- https://mellowpine.com/cnc/best-cnc-spindles/<br />
  Presents some specific spindles and what they're suitable for.
- https://en.wikipedia.org/wiki/Collet#ER_collets<br />
  Just some background info on ER collets, for the mechanically challenged (like me).

### Linear Guides

I figure that within the constraints of a home-built machine, linear rails are the best I can practically do in this category. I've decided to focus my research on those, and see where that leaves me budget-wise. I'll revisit later, if necessary.

Information from online shops:

- ARC/HRC: https://www.dold-mechatronik.de/Profilschienenfuehrungen-ARC-HRC
  - data sheet: https://www.dold-mechatronik.de/documents/Datenblaetter/Linearfuehrungen/Datenblatt-Linearfuehrung-ARC-HRC.pdf
  - seal (Dichtung): S or B
    - S: seals better; recommended for dirty environments.
    - B: less friction
  - pretension (Vorspannung): VC, V0, V1, V2
    - VC and V0 have play; only available in lower quality classes (H, N)
    - V1 and V2 don't, but have more friction
    - V1 might be a good compromise, is my initial impression
    - HRC has higher tension than ARC, in the equivalent classes
  - precision class (Genauigkeitsklassen): N, H, P, SP, UP
    - recommended for CNC mills: N to P, or H to SP for more precision
  - bearing cage (Kugelkette): should make sure it's included, unless price is prohibitive
  - standard lengths, minimum: 300mm
  - series:
    - ARC-M: compact, narrow
      - I assume I'll have two rails in every axis, so narrow carriages should be fine. I assume broader ones make sense, if you have just one rail.
    - ARC-F: compact, flange
      - broader than M
    - HRC-M: high, narrow
      - more height above the rail
    - HRC-F: high, flange
      - relates to HRC-M as ARC-F relates to ARC-M
  - sub-series: Each of the previously presented series is further divided into more series. The distinction between those is the size of the carriage that fits a given size rail. Larger ones can take more force.
    - ARC-MS (S = small?): rail sizes from 15x15 to 28x27
    - ARC-MN (N = normal?): rail sizes from 15x15 to 53x46
    - ARC-ML (L = large?): rail sizes from 15x15 to 53x46
- BR: https://www.dold-mechatronik.de/Profilschienenfuehrungen-BR-Serie
  - Can't find a data sheet.
  - BR9 and BR12 available.
  - Only BR12 has some dimensions in the shop. Smaller compared to even the smallest ARC/HRC.
  - Sold in sets of one rail + 2 carriages, which is what I need anyway.
  - Probably not interesting to me, given the small size and lack of information compared to ARC/HRC.
- HG:
  - shop links:
    - DOLD Mechatronik
      - https://www.dold-mechatronik.de/HG-Profilschienenfuehrungen
    - HIWIN
      - https://www.sorotec.de/shop/CNC-Mechanik/lineartechnik/Lineartechnik--Hiwin/Fuehrungswagen/Baureihe-HGH-247/
      - https://www.sorotec.de/shop/CNC-Mechanik/lineartechnik/Lineartechnik--Hiwin/Fuehrungswagen/Baureihe-HGW/
      - https://www.sorotec.de/shop/CNC-Mechanik/lineartechnik/Lineartechnik--Hiwin/Fuehrungsschienen/Baureihe-HGH/
    - Sorotec
      - https://www.sorotec.de/shop/CNC-Mechanik/lineartechnik/sorotec-blue-line-936/
  - data sheet: https://www.dold-mechatronik.de/documents/Datenblaetter/Linearfuehrungen/Datasheet_HGH-HGW_de_210720_Dold.pdf
  - DOLD Mechatronik data sheet is much less detailed than ARC/HRC
  - HIWIN has much more detailed information. See below.
  - Size seems comparable to lower range of ARC/HRC, which I think fits my use case.
- LF: https://www.dold-mechatronik.de/Profilschienenfuehrungen-LF-Serie
  - Sizes seem comparable to lower end of ARC/HRC.
  - Barely anything available at DOLD Mechatronik.
  - The available rails are way too long.
  - Not data sheet.
- LSK: https://www.dold-mechatronik.de/Profilschienenfuehrungen-LSK-Serie
  - data sheet: https://www.dold-mechatronik.de/documents/Dold_LSK_Linearfuehrungen.pdf
  - Sizes seem comparable to lower end of ARC/HRC.
  - FL and GL recommended for machine tools.
  - FL seems lower, possible better suited.
  - pretention (Vorspannung): Z2 (hightest) recommended for machine tools
  - more information in there I didn't go through in detail
  - unclear how this is meaningfully different from ARC/HRC
- MGN:
  - shop links:
    - DOLD Mechatronik
      - https://www.dold-mechatronik.de/MGN-Miniatur-Linearfuehrungen-schmal
    - Sorotec
      - https://www.sorotec.de/shop/CNC-Mechanik/lineartechnik/Lineartechnik--Hiwin/Fuehrungswagen/mgn-baureihe/
      - https://www.sorotec.de/shop/CNC-Mechanik/lineartechnik/Lineartechnik--Hiwin/Fuehrungsschienen/baureihe-mgn-r/
  - much smaller; probably more suited to low-force stuff like printers
- MGW: https://www.dold-mechatronik.de/MGW-Miniatur-Linearfuehrungen-breit
  - MGN, but broad
  - probably unnecessary, compared to MGN, since I'm going for two rails per axis
- MR: https://www.dold-mechatronik.de/MR-Miniatur-Linearfuehrungen
  - data sheet: https://www.dold-mechatronik.de/documents/Datenblaetter/Linearfuehrungen/Datenblatt-Miniaturlinearfuehrung-MR.pdf
  - very small, comparable to MGN/MGW
  - nice data sheet, didn't look into it in detail
  - didn't see anything about applications; probably too small for a mill
- MRW: https://www.dold-mechatronik.de/MRW-Miniatur-Linearfuehrungen-breit
  - same data sheet as MR
  - wide version of MR
- MSB: https://www.dold-mechatronik.de/Profilschienen-MSB-Serie-PMI
  - data sheet: https://www.dold-mechatronik.de/documents/Datenblaetter/Linearfuehrungen/MSB-TE-E.pdf
  - data sheet is sparse
  - size comparable with lower end of ARC/HRC
- ST: https://www.dold-mechatronik.de/ST-Miniatur-Kurzhub-Linearfuehrung
  - shares a data sheet with MRW
  - specialized thing for short movements; probably not interesting


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

### Control Software

On the controller side I have the following priorities:

- Resist the temptation to do any custom software, or otherwise use something exotic. This might be an option in later builds, but I want to keep it simple for the first version.
- Be controllable from a regular PC, without requiring one to run. I don't want to add a screen/keyboard/mouse to the bill of materials, but I also don't want to dedicate a whole computer to control it.

I found the following options:

- [**LinuxCNC**](https://linuxcnc.org/): Runs on the Raspberry Pi:
  http://linuxcnc.org/docs/stable/html/getting-started/getting-linuxcnc.html

  The download page talks about interface cards. Not sure what specifically is required, but I found this list:
  http://wiki.linuxcnc.org/cgi-bin/wiki.pl?LinuxCNC_Supported_Hardware

  I haven't done much research, but the interface cards I saw were quite expensive. Overall, I get the impression that LinuxCNC is not suited for a budget-sensitive build.

- [**Machinekit**](https://www.machinekit.io/): I found it hard to understand whether this is suitable. What I can gather is that it runs on the BeagleBone Black: https://www.machinekit.io/docs/getting-started/machinekit-images-for-bbb/

  No idea what else is required to make it work. The information presented is not very approachable, and I'm not sure how much of it is outdated.

- [**grbl**](https://github.com/gnea/grbl): I've often heard about this being used for the kind of small-scale CNC machine I'm aiming to build. It's confusing though. There are two different repositories, neither actively developed, and lots of forks.

- [**grblHAL**](https://www.grbl.org/): Fork of gbrl, for 32-bit MCUs.

- [**FluidNC**](https://github.com/bdring/FluidNC): Looks promising. Has a list of supported hardware: https://github.com/bdring/FluidNC/wiki/Hardware-that-Runs-FluidNC

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

### Power Supply

As far as I can see, the machine needs the following kinds of power:

- **230V AC**: Since I'm in Europe, this is the input I'm dealing with. It's needed by the VFD for the spindle, the DC power supply, possibly the water pump.
- **3.3V - 5V DC**: This is the typical range for microcontrollers and many other kinds of electronics.
- **higher-voltage DC**: The stepper motors are going to need DC at a higher voltage than the controller.

So I'm going to need a power supply that turns AC into DC, and possibly something else to step the DC up or down to meet the different requirements. As for the power supply, I've often seen [Meanwell](https://www.meanwell.com/) in various sets and such. Unless there's a good reason not to, I might just stick to that.

### CAD Software

I'd love to use [Fornjot](https://www.fornjot.app/), but it'll be a while before it's ready for a project like this. I hope that I can migrate the project to Fornjot eventually.

Normally, I'd favor open source software, but since migration to Fornjot at a later stage is planned anyway, I don't see that as a priority. I've used OpenSCAD, FreeCAD, SolveSpace, and other open source options in the past, and I'd like to take this opportunity to try something new.

I'm focusing the research on options that are free (or have a free tier) and support Linux:

- [**OnShape**](https://www.onshape.com/): Looks highly professional. Would certainly be an interesting learning experience.
- [**SketchUp**](https://sketchup.com/): Also looks interesting, although the website presents architecture and furniture use cases, versus the mechanical assemblies showcased on the OnShape website. Hard to say how relevant this will be for this project.
- [**TinkerCAD**](https://www.tinkercad.com/): Looks least interesting, judging from the website, as it stresses the beginner/education use case. Doesn't mean that it won't be more than capable enough for this project though.

### Enclosure

The design goal for the whole machine is to have it run in an apartment environment, if at all possible, without creating the kind of noise that would be unacceptable in such an environment. An enclosure is going to be critical to achieve that.

References:

- https://www.youtube.com/watch?v=1zIFWG3X1DU
