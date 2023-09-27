#!/bin/bash

# simple orbit
# mass=(10000 1)
# x=(0 0)
# y=(0 10000)
# vel_x=(0 10000)
# vel_y=(0 0)
# ticks=1000

# our separate ways
mass=(100000000 100000001 100000002)
x=(0 -100000000 100000000)
y=(100000000 -100000000 -100000000)
vel_x=(30000000 0 -30000000)
vel_y=(0 30000000 0)
ticks=4000

# slow dance
# mass=(100000000 100000001 100000002)
# x=(50000000 -70000000 -20000000)
# y=(50000000 -50000000 60000000)
# vel_x=(30000000 0 0)
# vel_y=(0 30000000 -30000000)
# ticks=8000

# figure of eight
# mass=(100000000 100000001 100000002)
# x=(-97000436 0 97000436)
# y=(24208753 0 -24208753)
# vel_x=(46620368 -93324973 46620368)
# vel_y=(43236573 -86473146 43236573)
# ticks=4000

# double orbit
# mass=(40000 40001)
# x=(0 0)
# y=(10000 -10000)
# vel_x=(10000 -10000)
# vel_y=(0 0)
# ticks=4000

address=$1

mass_string="["$(IFS=, ; echo "${mass[*]}")"]"
x_string="["$(IFS=, ; echo "${x[*]}")"]"
y_string="["$(IFS=, ; echo "${y[*]}")"]"
vel_x_string="["$(IFS=, ; echo "${vel_x[*]}")"]"
vel_y_string="["$(IFS=, ; echo "${vel_y[*]}")"]"

cast send --gas-limit 12000000 --private-key $PRIV_KEY --rpc-url $RPC_URL $address "mint(uint64[],int64[],int64[],int64[],int64[],uint32)" $mass_string $x_string $y_string $vel_x_string $vel_y_string $ticks

# some possible errors
# 0x7e273289 ERC721NonexistentToken(uint256 tokenId);
# 0x64283d7b ERC721IncorrectOwner(address sender, uint256 tokenId, address owner)
# 0x73c6ac6e ERC721InvalidSender(address sender)
# 0x64a0ae92 ERC721InvalidReceiver(address receiver)
# 0x177e802f ERC721InsufficientApproval(address operator, uint256 tokenId)
# 0xa9fbf51f ERC721InvalidApprover(address approver)
# 0x5b08ba18 ERC721InvalidOperator(address operator)

