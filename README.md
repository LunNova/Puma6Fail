# Puma 6 fail demo

Tool to demonstrate issue from this post found by mackey: https://www.dslreports.com/forum/r31377755-

DoS occurs in either direction - UDP from LAN to WAN or WAN to LAN.

Testing through a local Virgin Media Super Hub 3 modem:

    1mbps/2000pps   causes ~20ms average latency rise with 200 maximum
    2mbps/4000pps   causes ~200ms average latency and 65% packet loss
    3mbps/6000pps   causes ~250ms average latency and 85% packet loss

![Smokeping graph while testing](https://i.imgur.com/eshENJE.png)
