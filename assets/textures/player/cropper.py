from PIL import Image, ImageChops
import os

def crop_image(input_file, output_file):
    image = Image.open(input_file)
    image = image.convert("RGBA")

    # Find the cropping box
    bg = Image.new(image.mode, image.size, (255, 255, 255, 0))
    diff = ImageChops.difference(image, bg)
    diff = ImageChops.add(diff, diff, 2.0, -100)
    bbox = diff.getbbox() # left, upper, right, lower

    if bbox:
        width, height = bbox[2] - bbox[0], bbox[3] - bbox[1]
        size = max(width, height)

        # Create a square background with a transparent alpha channel
        cropped = Image.new("RGBA", (size, size), (255, 255, 255, 0))
        
        # Calculate the position to place the cropped image
        left, upper = (size - width) // 2, (size - height) // 2
        right, lower = left + width, upper + height

        # Paste the cropped image onto the square background
        cropped.paste(image.crop(bbox), (left, upper, right, lower))

        pixels_cropped_left = bbox[0] - ((size - width) // 2)
        pixels_cropped_right = image.width - pixels_cropped_left - size
        pixels_cropped_top = bbox[1] - ((size - height) // 2)
        pixels_cropped_bottom = image.height - pixels_cropped_top - size

        scale = min( size / image.width, size / image.height )
        offset = {
            'x': (pixels_cropped_left - pixels_cropped_right) / image.width,
            'y': (pixels_cropped_bottom - pixels_cropped_top) / image.height
        }
        print(f"{input_file}\tscale: {scale}\toffset: ({offset['x']}, {offset['y']})")
        cropped.save(output_file)

def main():
    input_dir = "./raw/"
    output_dir = "./"

    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    for filename in os.listdir(input_dir):
        if filename.endswith(".png"):
            input_file = os.path.join(input_dir, filename)
            output_file = os.path.join(output_dir, filename)

            # Crop and save the new image
            crop_image(input_file, output_file)

if __name__ == "__main__":
    main()