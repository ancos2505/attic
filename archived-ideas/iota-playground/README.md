# iota-playground



## Install openssh 

```sh
sudo apt-get update && sudo apt-get install openssh-server tmux musl-tools -y
```

## Run Openssh in user context
```sh    
ssh-keygen -A -f $HOME/.sshd
mkdir -p /home/vscode/.sshd/etc/ssh/
/usr/sbin/sshd  -D -p 10022 -h $HOME/.sshd/etc/ssh/ssh_host_ed25519_key
```