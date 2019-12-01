import requests

API_ENDPOINT = 'https://api.vexdb.io/v1'

class InteractiveRepl:
    """

    config is a dictionary
    {
        default_team: string
    }

    """
    def __init__(self, config):
        pass
    def eval(self, string):
        pass
    def lookup(self, key):
        pass
    def prompt(self):
        pass

s = requests.Session()

# Todo implement configuration file

config = {
    "default_team": "750B"
}

repl = InteractiveRepl(config)

input = repl.prompt()
while not input.lower() == 'exit':
    repl.eval(input)
    input = repl.prompt()