import svgwrite
import cairosvg
from io import BytesIO
from pygame import image, Surface
from xml.etree.ElementTree import fromstring as xml_fromstring
from xml.etree.ElementTree import parse as xml_parse
from svgpathtools import svg2paths, wsvg

def load_svg(file_path:str) -> str:
    with open(file_path, 'r') as file:
        svg_data = file.read()

    return svg_data

def svg_to_png(dwg : svgwrite.Drawing) -> Surface:
    # Save the SVG
    svgdata = BytesIO()
    svgdata.write(dwg.tostring().encode('utf-8'))
    svgdata.seek(0)

    # Rasterize the SVG using cairosvg
    surface_data = BytesIO()
    cairosvg.svg2png(file_obj=svgdata, write_to=surface_data)
    surface_data.seek(0)

    #Convert the rasterized image to a pygame surface
    return image.load(surface_data)

def generate_png(svg_file:str) -> Surface:
    # Create vector graphics
    dwg = svgwrite.Drawing(size=(64, 64))
    svg_data = load_svg(svg_file)

    # parse svg xml element from svg data string
    svg_element = xml_fromstring(svg_data)
    print(f'xml_fromstring: {svg_element}')
    #dwg.add(svg_element)

    paths, attributes = svg2paths(svg_file)
    print(f'paths: {paths} attributes: {attributes}')
    for path, attr in zip(paths, attributes):
        style = attr.get('style')
        fill = attr.get('fill', 'none')
        stroke = attr.get('stroke', 'black')
        stroke_width = attr.get('stroke-width', 1)
        dwg.add(dwg.path(d=path.d(), fill=fill, stroke=stroke, stroke_width=stroke_width, style=style))
    return svg_to_png(dwg)

'''
def svg_generate_ufo() -> pygame.Surface:
    return generate_png('ufo.svg')
'''