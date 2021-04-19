# The Enterprise System Administration Workshop

In this workshop we plan to 

1. Deploy a static website using devops principles
2. Deploy a dynamic website with an api with devops principles 

Notice that while the website, and the api are very simple, something that would
be trivial to deploy. These methods are not for these websites, these are mean't
to be applied to enterprise level apps on a large or small scale. So apps that
while they don't have to be massive apps, they are generally mean't to be apps
that teams of people would work on.

## Prerequisites

In terms of knowledge it would be helpful but not necessarily required to knowledge

- Basic unix/linux file structures and directory structures (not essential)
- Basic unix/linux commands (just look them up if you struggle)
- Vim or Vi text editing (can also use the nano text editor, but vscode is not 
  gonna help)

In terms of what software would be required to follow along in case you want to
follow along which is not required, can just listen.

1. A github account connected to your university or instituition (see 2)
2. Would need the student developer pack from github, should take about
   three seconds and a university email to apply for one.
3. Namecheap domain (free with student developer pack and a .me extension) Look up
   instructions on github student pack and google.
4. Some vps (digital ocean droplet recommended) just the account needed setting
   them up will be covered, also doing these things are provider agnostic, i'll 
   probably use digital ocean, but i'll try provide instructions for all 
   platforms
5. You NEED no way of getting around, some sort of unix shell. Instructions
   below.

### Windows Unix Shell

In windows there are many options for getting unix shells. Putty is one, but 
not recommended for system administration work. The two most recommended 
options are to either use a Virtual Machine, but they can be quite tedious
to set up, or you can use wsl. It is recommended that you use wsl.

WSL stands for windows subsytem for linux, and is essentially a linux kernel
embedded in windows allowing windows users access to the linux cli.

In order to get it follow this guide https://docs.microsoft.com/en-us/windows/wsl/install-win10

I would personally recommend downloading either Debian, Fedora or openSUSE for
WSL.

Debian has the most available packages, but you will only be a few here. 
OpenSUSE Leap, is well regarded as one of the best linux distros with good
default tools which would probably be my own personal choice. Then Fedora is
very similar to CentOS which is what we will be using as the deployment
environment, generally it is a good idea to have your development environment
be as close to your deployment environment as possible.

The default packages installed with wsl should be fine.

### MacOS

Make sure ssh is installed and enabled, just the client, not the server.

It is recommended to install vim or a cli text editor.

It is also recommended to install rsync for sysadmin work even if it isn't 
goint to be user here.

### Linux

See the macos for packages, but everything else should be fine.

## Introduction: What is devops and continuous delivery

From what I have read on the internet, most sys admins describe devops as a way
of life.

Now devops is seperate from the agile methodology but they often are used 
together and I find that they are easier to explain together.

Before the way software used to be developed is you would have a phase of 
development, then you would extensively test the app, and then you would release
(deploy) the app. These would happen in big increments of months or years and
follow a strict plan or schedule, then after this process you would discard and
move onto the second version, often a complete rewrite. They would often have a 
support period where they have hotfixes and patches but rarely big rewrites or 
new features often resulting in overall outdated and less quality in software.

The agile methodology takes the same simple idea of having a 
development -> test -> deploy cycle but instead these happen in smaller 
increments, and often have not quite extensive testing, then they wait for new 
issues to arise whether it be from the development point of view, or the users
point of view, and then repeat the cycle. It's agile because it is a cycle of 
feedback and incremental versions.

While in theory agile works, in practice, having to redeploy the app every 
increment or version or redo testing is a tedious and not worth while job. It 
makes the agile methodology fall short. Devops and continuous delivery is a 
process used to automate the process of deployment and testing. These convert
the testing and the release phases of agile into what we would call a devops 
pipeline.

A devops pipeline looks like.

Push from source control -> Staging -> Deploy

Where the first step is the development process.

Staging is where you build the app, extensively test it, then you create 
what we call artifacts such as the actual release binaries of the app.

Deployment is the task of configuring the server for production, and making it
accessible on the internet, taking the artifacts from the build process.

You can easily apply devops to nearly all your projects, being on a university 
level or on a professional level but depending on the amount of enterpriseness 
you might change your setup for example you could use github actions for open 
source or university development as your dev ops pipeline, or you might use 
ansible and jenkins like we will for something more enterprizy.

## Steps to setup postgres

1. Install postgres
2. switch users with `sudo -iu postgres`
3. Create a new user `exampleuser123` with `createuser --interactive` and set
   all permissions to none.
4. Create a new database with `createdatabase` called `todo_db`
5. Go into psql with `psql` and type `ALTER ROLE exampleuser123 WITH PASSWORD 'password123';`
6. Now run `GRANT ALL PERMISSIONS ON DATABASE todo_db TO exampleuser123;` and quit with ctrl+D


## PART 1: Setting up a VPS CentOS Instance

### Create accounts and 
First create some account of a vps service, three are recommended
  
- Digital Ocean Droplet
- Google CentOS Instance
- Microsoft Azure

### Create a VPS Instance

#### Digital Ocean

1. Get started with a droplet after you log in

2. Select CentOS 7.6 as the distribution

3. Select a relevant storage size, cpu and memory, as long as they are standard
   shared cpu droplets, it doesn't matter which one.

4. Choose any region, I shall be choosing singapore because it is closest.

5. Choose Password instead of ssh, and don't select User Data, although if you
   are familiar with these options then you can go ahead and select them.
   Select a root password and continue.

6. One droplet with any available hostname, shall be using the default hostname

7. Make sure to select the backups option.

8. Wait for the droplet to be created.

### Setting up your website url

We will be using namecheap for three reasons, 1. It's free, 2. It has whois guard protection
meaining your private information stays private, and 3. It really doesn't matter which
provider you use since they are all pretty much the same, and we won't be using
namecheaps dns anyway.

1. Log into namecheap.com and navigate to Dashboard.
2. Navigate to your domain, and select manage
3. Under name servers, change to custom dns, and set the nameservers to the following

```
ns1.digitalocean.com
ns2.digitalocean.com
ns3.digitalocean.com
```

4. Log back into digital ocean and on the `...` next to your droplets name, select add a domain

5. Add these records

```
CNAME Record:
hostname: www 
is an alias of: @

A Record:
hostname: api
will direct to: your droplet

A Record:
hostname: jenkins
will direct to: your droplet
``` 

### Setting up SSH

Setting up SSH is pretty easy

1. run the command `ssh-keygen` and simply press enter through all the options.
   note that if you would like to set a password that is fine.
2. Next add ssh to your github account, click your profile icon in the top right,
   select `settings`, then under SSH and GPG keys click New SSh key on the top right.
   Copy the your key with the command `cat .ssh/id_rsa.pub` this will print your
   PUBLIC key to the terminal, as `cat` prints files to the terminal, and `.ssh/id_rsa.pub`
   is the location of where your key is stored. ssh keys have a private and a public
   component. The private is the key and only you can access it, the public key is like
   providing the lock that they key fits so never give your private key away.
3. now in your server instance, find what the servers public ip is (remember to
   provide instructions for this) and run the command `ssh-copy-id root@yourdomain` where
   the server ip is the public ip address to your server. (Note: sometimes it takes a while
   for domains to get registered when switching nameservers, in that case use the ip)

We done for now.

### Users and permissions

In linux we have users and groups

Linux contains a simple permissions model, each service has permission information, they
can be read, written, or executed (like opening up an app in windows). You can
view permission with the `ls -a` command, it appears with something of the form of
`drwxrw-r--`, the first letter `d` tells us whether it is a directory or not. The
next three characters are in the form of `rwx` where `r` is the read permission, `w`
is the write permission, `x` is the execute permission, and `-` says they do not have
the given permission.

Example:

`rw-` - Read and Write permissions but no execute.

`r-x` - Read and execute but no write permission.

Note that `rwx` is repeated three times, the first time is the read, write and execute
permissions for a user, the second is for a group, and the third is for the system.

In linux we want to restrict access as much as possible to prevent things from
having access to other things they shouldn't.

In linux a user is like a user of an account like in windows or mac, and a group is just
a group of users. When we add permissions to a specific user, only that user can access
that resource, but if we need some users to access a resource but others not be able to
access a given resource, then we can create a group. 

Note on `gpasswd` vs `usermod`

There are two ways to add a user to a group, `usermod -aG somegroup username` and `gpasswd -a username somegroup`

The difference is subtle, but `usermod` modifies the users configuration with options such
as changing there default terminal shell, or adding and removing groups.

`usermod` has two options that we will look at `-G` which gives a user certain groups, and `-a`,
which adds the pre-existing groups that a user had to the user. Now the problem is if you
accidentally forget the `-a` flag, you end up removing the groups a user alread had, and
so this can be considered more risky, in practice this rarely happens. On the otherhand
`gpasswd` can either add or remove groups so it is therefore a safer option.

1. run the command `adduser exampleuser` where the user is your username in my case
   emendoza.
2. set a passwd for your user with the command `passwd exampleuser`
3. run the command `gpasswd -a exampleuser wheel` this adds our user to the wheel
   group. In Linux, the wheel group is like the admin role in windows.
4. Logout of ssh with Ctrl+D or type `exit` into the cli.
5. Now repeat the command `ssh-copy-id exampleuser@yourdomain` notice we are now
   using the example user instead of root.

### Install required software

1. Run command `sudo yum install vim-enhanced`

### Change ssh settings

We will use vim for this, if you are unfamiliar use replace `vim` with `nano`,
but remember, as described in the Unix and Linux System Administration Handbook
System administrators will judge you so do it descretely if you plan to deploy
infront of other system admins.

1. enter the command `sudo vim /etc/ssh/sshd_config`
2. change the line `PermitRootLogin yes` to `PermitRootLogin no`
3. change the line `PasswordAuthentication yes` to `PasswordAuthentication no`
4. quit vim with `:wq`
5. reload ssh daemon with `sudo systemctl reload sshd`
6. Test it works before you exit by entering a new terminal and typing `ssh exampleuser@yourdomain`

### Firewall

A server works by exposing all ports available to the outside world. As system administrators one
of our primary jobs is to restrict the access of the outside world to be able to only access what
they need. So we can directly access our server, but nobody else should be able to, similarly when
we serve our website, we want people only to be able to access what they need to see the website,
we don't want them to be able to access anything else. A firewall makes it so that people can only
access ports that we specify, with all other ports still being accessible internally.

#### About firewalld
for this workshop we will use firewalld, it works by having default zones, a zone is an area that
the network runs in, for example, you might have a zone for local computers in an office, and a zone
for computers outside in the public, or a zone for just the computer specifically. Each zone might
have certain services enabled such as you might have http enabled publically for everyone, but only
have ssh enabled for users within the local network.

More information about firewalld can be found by googling archwiki firewalld, and you can see what
commands are available by reading the manual with `man firewall-cmd`.

1. Run command `sudo yum install firewalld`
2. start firewall service with `sudo systemctl start firewalld`
3. add ssh to the firewall permissions list with `sudo firewall-cmd --permanent --add-service=ssh`
4. type `sudo firewall-cmd --relaod` to enable changes
5. similar to before open up a new terminal and see if you can still access the server.
6. if it all works enable changes with `sudo systemctl enable firewalld`

### ntp timezones

### Jenkins

### Nginx

### Certbot and https

### Ansible

## Part 2: Deploy a static website with nginx, http2 and https with ansible+jenkins

### More Users, Groups and SELinux

## Part 3: Configure building, testing and deployment for api with jenkins

### Service Daemons, and more SELinux
