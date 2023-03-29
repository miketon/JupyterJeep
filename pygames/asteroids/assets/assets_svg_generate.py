import io
import svgwrite
import cairosvg
import pygame

def svg_to_png(dwg : svgwrite.Drawing) -> pygame.Surface:
    # Save the SVG
    svgdata = io.BytesIO()
    svgdata.write(dwg.tostring().encode('utf-8'))
    svgdata.seek(0)

    # Rasterize the SVG using cairosvg
    surface_data = io.BytesIO()
    cairosvg.svg2png(file_obj=svgdata, write_to=surface_data)
    surface_data.seek(0)

    #Convert the rasterized image to a pygame surface
    return pygame.image.load(surface_data)

def svg_generate_ufo() -> pygame.Surface:
    # Create vector graphics
    dwg = svgwrite.Drawing(size=(64, 64))
    dwg.add(dwg.circle(center=(32, 32), r=32, fill='red'))
    return svg_to_png(dwg)