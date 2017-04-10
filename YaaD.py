#!/usr/bin/env python3

import re
import os
import sys
from subprocess import call
from html.parser import HTMLParser
import requests
from termcolor import cprint

# Root page parser class
class AnimeSiteParser(HTMLParser):
    episodes = []
    def handle_starttag(self, tag, attrs):
        if tag == 'a':
            for attr in attrs:
                if 'href' in attr[0]:
                    link = attr[1]
                    if link.find('https://anime-odcinki.pl/anime/' + animeName) != -1 and re.search(r'\d+$', link):
                        self.episodes.append(link)

# Children's pages parser class
class AnimeEpisodeSiteParser(HTMLParser):
    encryptedVideoLinks = []
    def handle_starttag(self, tag, attrs):
        if tag == "div":
            for attr in attrs:
                if attr[0] == "data-hash":
                    self.encryptedVideoLinks.append(attr[1])


# $1 is name of anime to downloadd
animeName = "-".join(sys.argv[1].lower().split())
url = "https://anime-odcinki.pl/anime/" + animeName
header = {
    'User-Agent' : 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/57.0.2987.133 Safari/537.36',
    'Accept' : 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8'
}

session = requests.Session()

# get root page
req = session.get(url, headers=header)
# parse root page
animeSiteParser = AnimeSiteParser()
animeSiteParser.feed(req.text)
# search for a links to episodes
for episode in animeSiteParser.episodes:
    cprint(episode, 'green')
animeSiteParser.episodes.reverse()
for episode in animeSiteParser.episodes:
    req = session.get(episode, headers=header)
    animeEpisodeSiteParser = AnimeEpisodeSiteParser()
    animeEpisodeSiteParser.feed(req.text)
    with open('./communicationFile', 'w+') as communicationFile:
        communicationFile.write(animeEpisodeSiteParser.encryptedVideoLinks[0])
    call(['node', './decryptVideoLink.js'])
    os.system('node ./decryptVideoLink.js')
    with open('./communicationFile', 'w+') as communicationFile:
        videoLink = communicationFile.read()
    cprint(videoLink, 'blue')
    os.system('youtube-dl \"%s\"' % animeEpisodeSiteParser.videos[0])
