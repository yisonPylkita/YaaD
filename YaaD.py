#!/usr/bin/env python3

from html.parser import HTMLParser
import requests
from termcolor import cprint
import os
import sys

# Root page parser class
class MyHTMLParser(HTMLParser):
    links = []
    def handle_starttag(self, tag, attrs):
        if tag == 'a':
            for attr in attrs:
                if 'href' in attr[0]:
                    self.links.append(attr[1])

# Children's pages parser class
class MyHTMLParserNext(HTMLParser):
    links = []
    def handle_starttag(self, tag, attrs):
        if tag == "iframe":
            for x in attrs:
                if type(x[1]) == str and not x[1].find('http://vk') == -1:
                    self.links.append(x[1])

url = sys.argv[1]
header = {'User-Agent' : 'Mozilla/5.0 (X11; Linux i686; rv:33.0) Gecko/20100101 Firefox/33.0'}

# get root page
req = requests.get(url, headers=header)
# parse root page
parser = MyHTMLParser()
parser.feed(req.text)
episodes = []
# search for a links to episodes
for x in parser.links:
    if not x.find('http://anime-odcinki.pl/viewpage.php?page_id=') or not x.find('/infusions/video/video.php?id=') or not x.find('/viewpage.php?page_id='):
        episodes.append(x)
        cprint(x, 'green')
episodes.reverse()
for x in episodes:
    if 'http://anime-odcinki.pl/viewpage.php?page_id=' in x:
        req = requests.get(x, headers=header)
    else:
        req = requests.get('http://www.anime-odcinki.pl' + x, headers=header)
    parser = MyHTMLParserNext()
    parser.feed(req.text)
    cprint(parser.links[0], 'blue')
    os.system('youtube-dl \"%s\"' % parser.links[0])
