# encoding: utf-8
# -*- mode: ruby -*-
# vi: set ft=ruby :
VAGRANT_BOX = 'bento/ubuntu-18.04'
VM_NAME = 'ectk'
VM_USER = 'vagrant'
HOST_PATH = '.'
GUEST_PATH = '/container/ectk'
#VM_PORT = 8080
Vagrant.configure(2) do |config|
  config.vm.box = VAGRANT_BOX
  config.vm.hostname = VM_NAME
  config.vm.provider "virtualbox" do |v|
    v.name = VM_NAME
    v.memory = 2048
  end
  config.vm.network "private_network", type: "dhcp"
  config.vm.synced_folder HOST_PATH, GUEST_PATH
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install -y git
    apt-get install -y docker
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    apt-get install -y build-essential
    apt-get update
    apt-get upgrade -y
    apt-get autoremove -y
    mkdir /container
    rsync -aAXv / --exclude={"/dev/*","/tmp/*","/run/*","/proc/*","/sys/*","/mnt/*","/media/*","/lost+found","/home/*","/container"} /container
  SHELL
end