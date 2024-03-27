from PIL import Image, ImageChops
import os

raw_images = {}
processable_images = {}
multies = ['head', 'legs']

def preprocess_image(input_file, output_file):
    image = Image.open(input_file)
    image = image.convert("RGBA")

    # Find the cropping box
    bg = Image.new(image.mode, image.size, (255, 255, 255, 0))
    diff = ImageChops.difference(image, bg)
    diff = ImageChops.add(diff, diff, 2.0, -100)
    bbox = diff.getbbox() # left, upper, right, lower
    name = input_file.split("/")[-1].split(".")[0]

    raw_images[input_file] = (name, image, bbox, output_file)

def crop_image(entry, bbox):
    (name, image, output_file) = entry
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
    print(f"{name}\tscale: {scale}\toffset: ({offset['x']}, {offset['y']}) bbox: {bbox}")
    #print(f"{name}\t({offset['x']}, {offset['y']})")
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
            # Preprocess imagese
            preprocess_image(input_file, output_file)
    for key in raw_images.keys():
        #file_name = key.split("/")[-1].split(".")[0]
        name = raw_images[key][0]
        bbox = raw_images[key][2]
        extracted_img = (
            raw_images[key][0],
            raw_images[key][1],
            raw_images[key][3]
        )
        for multi in multies:
            if multi in name:
                if multi in processable_images.keys():
                    old_bbox = processable_images[multi][0]
                    new_bbox = (
                        min(old_bbox[0], bbox[0]),
                        min(old_bbox[1], bbox[1]),
                        max(old_bbox[2], bbox[2]),
                        max(old_bbox[3], bbox[3])
                    )
                    processable_images[multi] = (new_bbox, processable_images[multi][1] + [extracted_img])
                else:                    
                    processable_images[multi] = (bbox, [extracted_img])
                break
        else:
            processable_images[name] = (bbox, [extracted_img])
    #print(processable_images)       
    for multi in processable_images.keys():
        img_data = processable_images[multi]
        bbox = img_data[0]
        #print(img_data)
        for entry in img_data[1]:
            crop_image(entry, bbox)

if __name__ == "__main__":
    main()