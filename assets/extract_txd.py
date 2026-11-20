import os
import struct
from PIL import Image

def unpack_rgb565(c):
    r = ((c >> 11) & 0x1F) * 255 // 31
    g = ((c >> 5) & 0x3F) * 255 // 63
    b = (c & 0x1F) * 255 // 31
    return (r, g, b)

def decode_dxt1(data, width, height):
    img = Image.new("RGBA", (width, height))
    pixels = img.load()
    
    pos = 0
    for by in range(0, height, 4):
        for bx in range(0, width, 4):
            c0, c1, lookup = struct.unpack_from("<HHI", data, pos)
            pos += 8
            
            r0, g0, b0 = unpack_rgb565(c0)
            r1, g1, b1 = unpack_rgb565(c1)
            
            colors = [
                (r0, g0, b0, 255),
                (r1, g1, b1, 255),
            ]
            if c0 > c1:
                colors.append(((2 * r0 + r1) // 3, (2 * g0 + g1) // 3, (2 * b0 + b1) // 3, 255))
                colors.append(((r0 + 2 * r1) // 3, (r0 + 2 * g1) // 3, (b0 + 2 * b1) // 3, 255))
            else:
                colors.append(((r0 + r1) // 2, (g0 + g1) // 2, (b0 + b1) // 2, 255))
                colors.append((0, 0, 0, 0))
                
            for py in range(4):
                for px in range(4):
                    code = (lookup >> (2 * (py * 4 + px))) & 3
                    if bx + px < width and by + py < height:
                        pixels[bx + px, by + py] = colors[code]
    return img

def decode_dxt3(data, width, height):
    img = Image.new("RGBA", (width, height))
    pixels = img.load()
    
    pos = 0
    for by in range(0, height, 4):
        for bx in range(0, width, 4):
            alpha_data = struct.unpack_from("<Q", data, pos)[0]
            pos += 8
            c0, c1, lookup = struct.unpack_from("<HHI", data, pos)
            pos += 8
            
            r0, g0, b0 = unpack_rgb565(c0)
            r1, g1, b1 = unpack_rgb565(c1)
            colors = [
                (r0, g0, b0),
                (r1, g1, b1),
                ((2 * r0 + r1) // 3, (2 * g0 + g1) // 3, (2 * b0 + b1) // 3),
                ((r0 + 2 * r1) // 3, (r0 + 2 * g1) // 3, (b0 + 2 * b1) // 3),
            ]
            
            for py in range(4):
                for px in range(4):
                    idx = py * 4 + px
                    code = (lookup >> (2 * idx)) & 3
                    alpha_4 = (alpha_data >> (4 * idx)) & 0xF
                    alpha = (alpha_4 * 255) // 15
                    if bx + px < width and by + py < height:
                        c = colors[code]
                        pixels[bx + px, by + py] = (c[0], c[1], c[2], alpha)
    return img

def extract_textures(txd_path, out_dir):
    os.makedirs(out_dir, exist_ok=True)
    with open(txd_path, "rb") as f:
        buf = f.read()
        
    pos = 0
    txd_id, txd_size, _ = struct.unpack_from("<III", buf, pos)
    pos += 12
    # Struct
    s_id, s_size, _ = struct.unpack_from("<III", buf, pos)
    pos += 12
    num_tex = struct.unpack_from("<H", buf, pos)[0]
    pos += s_size
    
    for i in range(num_tex):
        if pos >= len(buf): break
        chunk_id, chunk_size, _ = struct.unpack_from("<III", buf, pos)
        pos += 12
        end_chunk = pos + chunk_size
        
        # Native Texture Struct
        pos += 12 # skip struct header
        s_buf = buf[pos:end_chunk]
        name = s_buf[8:40].split(b"\x00")[0].decode("ascii", "ignore")
        meta = s_buf[72:]
        w, h, depth, num_mips = struct.unpack_from("<HHBB", meta, 8)
        data_size = struct.unpack_from("<I", meta, 16)[0]
        pixel_data = meta[20:20+data_size]
        
        out_path = os.path.join(out_dir, f"{name}.png")
        print(f"Extracting {name} ({w}x{h}, data_size={data_size})...")
        
        # Check compression
        if data_size == (w * h) // 2: # DXT1
            img = decode_dxt1(pixel_data, w, h)
            img.save(out_path)
            print(f"  Saved DXT1 {out_path}")
        elif data_size == (w * h): # DXT3
            img = decode_dxt3(pixel_data, w, h)
            img.save(out_path)
            print(f"  Saved DXT3 {out_path}")
        else:
            print(f"  Unknown format with data_size={data_size}")
            
        pos = end_chunk

extract_textures("/home/vmko/Games/reVC-lawlessness/models/fronten1.txd", "/home/vmko/.gemini/antigravity/scratch/neon-engine/assets")
extract_textures("/home/vmko/Games/reVC-lawlessness/models/fonts.txd", "/home/vmko/.gemini/antigravity/scratch/neon-engine/assets")
