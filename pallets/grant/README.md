# Grant Pallet

## Version: 0.0.1

 - [`Config`]
 - [`Pallet`]

 ## Overview

[Requirements](https://hackmd.io/dXVncf-SRzKAw8iqsoGCYA)

Every system without external entropy eventually leads to centrilization.
Grant pallet is able to grant entropy to the system by randomly assigning tokens to requesting accounts.




The Grant Pallet is used to Grant tokens to new AccountIDs.
In order to create Profile, Tasks, Organizations users need initial tokens. 
These tokens can be granted through a grant pallet.

The grants are issued in random fashion, such that requesters are awarded tokens in a random manner.
The intention is that initially, when there are only few users of the platform, every grant_request is
automatically approved. However, later on when the application reaches more use, grants are offered randomly
to requesting accounts. 
 	
 The Process is envisioned as follows:
 1. Anyone can send Funds into a Treasury Account. The Treasury account is used to distribute grant rewards.
 2. Anyone can request a grant each block.	
 3. Each block a grant is offered randomly to selected grant requester.
 
 	
 ## Interface

 ### Public Functions
  -  request_grant()
		Function used to request grants.

  -  transfer_funds()
		Function used to transfer funds into a Treasury account. Anyone can transfer into Treasury.

  -  winner_is()
		Function that announces the winner of the block.

 ## Related Modules
