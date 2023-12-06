cd github
echo "==================="
echo "Program Beginning..."
echo "==================="

# import the necessary modules
import paramiko

# set up the ssh connection
ssh = paramiko.SSHClient()
ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
ssh.connect(hostname="remote_hostname_or_IP", port=22, username="username", password="password")

# fetch and print market data
stdin, stdout, stderr = ssh.exec_command("kucoin-cli markets")
print(stdout.read().decode())

ssh-keygen -t rsa -b 4096 -C "your_email@example.com"

# You can create a port using Secure Shell (SSH) with the following commands:

# Generate an SSH Keypair:
# ssh-keygen -t rsa -b 2048 -f ~/.ssh/[KEY_NAME]
# Create an SSH configuration file (if it does not already exist):
# touch ~/.ssh/config
# Open the configuration file:
# nano ~/.ssh/config
# Enter the following contents into the configuration file, replacing HOST_NAME with the desired port name and PORT_NUMBER
# Copy
# Insert
# New
# with the port number to listen on:
# Host HOST_NAME
# Port PORT_NUMBER
# IdentityFile ~/.ssh/[KEY_NAME]
# Save and close the configuration file:
# Ctrl + X
# Start the SSH server:
# ssh -fNT -i ~/.ssh/[KEY_NAME] HOST_NAME

# To set up a host using SSH, you can use the following steps:

# Generate an SSH key with ssh-keygen -t rsa
# Copy the newly created public key to the server by running this command: ssh-copy-id username@hostname
# Login to the remote system using the command: ssh username@hostname
# Create a port on the remote system using the command: sudo firewall-cmd --zone=public --add-port=PORTNUMBER/tcp
# Enable the newly created port using the command: sudo firewall-cmd --zone=public --add-port=PORTNUMBER/tcp --permanent
# Exit the SSH session using the command: exit