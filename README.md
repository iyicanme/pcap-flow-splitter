# pcap_flow_splitter

`pcap_flow_splitter` is a TUI program that groups packets in a packet capture file by flow.

Supported file types are `.cap`, `.pcap` and `.pcapng`.

## Usage

Program can be run from a terminal.

It will launch a TUI that can browse the filesystem to open capture files.

```shell
$ ./pcap_flow_splitter

 /home/iyicanme/Desktop ──────────────────────────────────────────────────────────────────────
   NAME
   love_letters_to_ferris
   packet_captures
   http.ccap
   dns.pcap
   radius.pcapng
   [↑] UP [↓] DOWN [ESC] EXIT [↵] OPEN [BACKSP] GO UP ────────────────────────────────────────
```

Program can be navigated using the controls on the screen.

Selecting a file switches to flow viewer.

```shell
 http.cap ────────────────────────────────────────────────────────────────────────────────────
[UDP] 203...:135 ↔ 237...:419 [TCP] 223...:200 ↔ 237...:177 [TCP] 99...:200 ↔ 237...:121
   #    DIRECTION                             TIMESTAMP                                 LENGTH
   1    →                                     0.000000000                               62
   2    ←                                     0.000911310                               62
   3    →                                     0.000911310                               54
   4    →                                     0.000911310                               533
   5    ←                                     0.001472116                               54
   6    ←                                     0.001682419                               1434
   7    →                                     0.001812606                               54
   8    ←                                     0.001812606                               1434
   9    →                                     0.002012894                               54
   10   ←                                     0.002443513                               1434
┌────────────────────────────────────────────────────────────────────────────────────────────┐
│Initiator:    237...:11277    Total size:   20695    Flow duration:              0.030393704│
│Respondent:   223...:20480    Average size: 608      Average inter-arrival time: 0.000921021│
│Protocol:     TCP             Minimum size: 54       Minimum inter-arrival time: 0.000000000│
│Packet count: 34              Maximum size: 1434     Maximum inter-arrival time: 0.012888533│
└────────────────────────────────────────────────────────────────────────────────────────────┘
 [↑] UP [↓] DOWN [←] PREVIOUS [→] NEXT [ESC] EXIT [BACKSP] CLOSE FILE ────────────────────────
```

Flow viewer displays the list packets of each flow and some statistics of the flow.

Flows navigated using the tabs.

Flow viewer for a capture file can be directly launched by running the program with `--file_path` flag

```shell
./pcap_flow_splitter --file_path ~/Desktop/http.cap
```

## Future work

Exporting flow to a file from flow viewer:
Extracting a single flow from a capture file was the original goal of this project.
Older iterations were able to do this, but during addition of TUI, it was removed.

UI improvements:
Currently, UI layout is not optimal. Resizing can make some information disappear.

Performance improvements:
While the program works smoothly as is, I know I left many free lunches on the table.

## Contribution

I ask any issues, discussions and improvements to be discussed through GitHub issues.
