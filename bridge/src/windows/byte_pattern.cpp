#include <cstdint>
#include <cstdio>
#include <vector>

extern "C"
{
  static const int kMaxBytes = 256;

  struct Segment
  {
    int pos;
    std::vector<uint8_t> bytes;
  };

  typedef std::vector<Segment> Pattern;

  Pattern *create_pattern(const char *pattern)
  {
    Pattern *dst = new Pattern();

    size_t size = strlen(pattern);
    Pattern &compiled = *dst;

    auto hex2dec = [](char hex) -> uint8_t {
      if (hex >= '0' && hex <= '9')
      {
        return hex - '0';
      }
      else if (hex >= 'A' && hex <= 'F')
      {
        return hex - 'A' + 0xA;
      }
      else if (hex >= 'a' && hex <= 'f')
      {
        return hex - 'a' + 0xA;
      }
      else
      {
        return 0; //invalid
      }
    };

    Segment seg;
    seg.pos = 0;
    seg.bytes.reserve(kMaxBytes);
    auto next = [&compiled, &seg](int pos) {
      if (seg.bytes.size())
      {
        compiled.push_back(seg);
        seg.bytes.clear();
      }
      seg.pos = pos;
    };

    auto push = [&seg](uint8_t byte) {
      if (seg.bytes.size() < kMaxBytes)
      {
        seg.bytes.push_back(byte);
      }
    };

    for (size_t i = 0; i < size - 1; i += 3)
    {
      uint8_t value;
      if (pattern[i] == '?')
      {
        next(i / 3 + 1);
      }
      else
      {
        value = hex2dec(pattern[i]) * 0x10 + hex2dec(pattern[i + 1]);
        push(value);
      }
    }
    next(0);

    return dst;
  }

  void destroy_pattern(Pattern *pattern)
  {
    delete pattern;
  }

  const void *find_pattern(Pattern *pattern, const void *range_begin, size_t size)
  {
    if (!pattern)
    {
      return nullptr;
    }

    if (pattern->size() == 0)
    {
      return nullptr;
    }

    const void *result = nullptr;
    const uint8_t *begin = reinterpret_cast<const uint8_t *>(range_begin);
    int range_pos = 0;
    for (size_t seg = 0; seg < pattern->size();)
    {
      Segment &s = pattern->at(seg);
      size_t seg_size = s.bytes.size();
      if (range_pos + s.pos + seg_size > size)
      {
        break; //not found
      }

      if (memcmp(&s.bytes[0], begin + range_pos + s.pos, seg_size) == 0)
      {
        if (seg < pattern->size() - 1)
        {
          ++seg;
        }
        else
        {
          result = begin + range_pos;
          break; //found all segments
        }
      }
      else
      {
        ++range_pos;
        seg = 0;
      }
    }
    return result;
  }

  bool match_pattern(Pattern *pattern, const void *range_begin, size_t size)
  {
    return find_pattern(pattern, range_begin, size) != nullptr;
  }
}