import importlib
import json
import pkgutil
import sys


def run_generator(generator_module):
    module = importlib.import_module(f"scripts.generators.{generator_module}",)
    scene = module.generate()
    json.dump(scene, sys.stdout, indent=4)

def list_generators():
    module = importlib.import_module("scripts.generators", )
    for _, name, _ in pkgutil.walk_packages(module.__path__):
        print(name)

def main():
    if len(sys.argv) > 1:
        run_generator(sys.argv[1])
    else:
        list_generators()
