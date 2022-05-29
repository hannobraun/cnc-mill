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

- OnShape: https://www.onshape.com/
  Looks highly professional. Would certainly be an interesting learning experience.
- SketchUp: https://sketchup.com/
  Also looks interesting, although the website presents architecture and furniture use cases, versus the mechanical assemblies showcased on the OnShape website. Hard to say how relevant this will be for this project.
- TinkerCAD: https://www.tinkercad.com/
  Looks least interesting, judging from the website, as it stresses the beginner/education use case. Doesn't mean that it won't be more than capable enough for this project though.

### Configuration

I believe that the following machine configurations are the strongest contenders:

- **Moving gantry:** Tool moves in all 3 axes.
- **Moving gantry:** Tool moves in x and z axes, table moves in y axis.

I have ruled out more exotic configurations for my first build (despite having lots of ideas), to reduce overall risk.

References:

- https://cncchronicle.com/fixed_or_moving_gantry_for_cnc_router/
  Compares the two configuration. Has a nice comparison table further down.

### Control Software

On the controller side I have the following priorities:

- Resist the temptation to do any custom software, or otherwise use something exotic. This might be an option in later builds, but I want to keep it simple for the first version.
- Be controllable from a regular PC, without requiring one to run. I don't want to add a screen/keyboard/mouse to the bill of materials, but I also don't want to dedicate a whole computer to control it.

I found the following options:

- LinuxCNC: https://linuxcnc.org/
  Runs on the Raspberry Pi:
  http://linuxcnc.org/docs/stable/html/getting-started/getting-linuxcnc.html

  The download page talks about interface cards. Not sure what specifically is required, but I found this list:
  http://wiki.linuxcnc.org/cgi-bin/wiki.pl?LinuxCNC_Supported_Hardware

  I haven't done much research, but the interface cards I saw were quite expensive. Overall, I get the impression that LinuxCNC is not suited for a budget-sensitive build.

- Machinekit: https://www.machinekit.io/
  I found it hard to understand whether this is suitable. What I can gather is that it runs on the BeagleBone Black: https://www.machinekit.io/docs/getting-started/machinekit-images-for-bbb/

  No idea what else is required to make it work. The information presented is not very approachable, and I'm not sure how much of it is outdated.

- grbl: https://github.com/gnea/grbl
  I've often heard about this being used for the kind of small-scale CNC machine I'm aiming to build. It's confusing though. There are two different repositories, neither actively developed, and lots of forks.

Based on my cursory research into this area, I think this might be the wrong approach. It might be better to search for easily available CNC controllers, and figure out which software to use for them from there.
