#!/bin/bash

ip netns add host1
ip netns add host2

ip link add host1-eth0 type veth peer name host2-eth0

ip link set host1-eth0 netns host1
ip link set host2-eth0 netns host2

ip netns exec host1 ip addr add 192.168.0.101/24 dev host1-eth0
ip netns exec host2 ip addr add 192.168.0.102/24 dev host2-eth0

ip netns exec host1 ip link set lo up
ip netns exec host2 ip link set lo up
ip netns exec host1 ip link set host1-eth0 up
ip netns exec host2 ip link set host2-eth0 up